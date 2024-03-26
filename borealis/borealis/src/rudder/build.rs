use {
    crate::{
        boom::{self, NamedType, NamedValue},
        rudder::{
            self, BinaryOperationKind, CastOperationKind, Context, Function, FunctionKind,
            ShiftOperationKind, Statement, StatementBuilder, StatementKind, Type,
        },
    },
    common::{intern::InternedString, HashMap},
    std::{cell::RefCell, cmp::Ordering, rc::Rc},
};

pub fn from_boom(ast: &boom::Ast) -> Context {
    let mut build_ctx = BuildContext::default();

    // DEFINITION ORDER DEPENDANT!!!
    ast.definitions.iter().for_each(|def| match def {
        boom::Definition::Enum { name, variants } => build_ctx.add_enum(*name, variants),
        boom::Definition::Union { name, fields } => build_ctx.add_union(*name, fields),
        boom::Definition::Struct { name, fields } => build_ctx.add_struct(*name, fields),
        boom::Definition::Let { bindings, .. } => {
            //todo handle body as like a setup fn or something?
            assert_eq!(1, bindings.len());
            let NamedType { name, typ } = &bindings[0];

            let typ = build_ctx.resolve_type(typ.clone());
            build_ctx.add_register(*name, typ);
        }
        // todo
        boom::Definition::Pragma { .. } => (),
    });

    ast.registers.iter().for_each(|(name, typ)| {
        let typ = build_ctx.resolve_type(typ.clone());
        build_ctx.add_register(*name, typ)
    });

    // need all functions with signatures before building
    ast.functions
        .iter()
        .for_each(|(name, definition)| build_ctx.add_function(*name, definition));

    build_ctx.build_functions();
    build_ctx.finalise()
}

#[derive(Default)]
struct BuildContext {
    /// Name of struct maps to the rudder type and a map of field names to field
    /// indices
    structs: HashMap<InternedString, (Rc<rudder::Type>, HashMap<InternedString, usize>)>,
    /// Name of union maps to the rudder type and a map of field names to field
    /// indices
    unions: HashMap<InternedString, (Rc<rudder::Type>, HashMap<InternedString, usize>)>,
    /// Name of enum maps to the rudder type and a map of enum variants to the
    /// integer discriminant of that variant
    enums: HashMap<InternedString, (Rc<rudder::Type>, HashMap<InternedString, u32>)>,

    /// Register name to type and offset mapping
    registers: HashMap<InternedString, (Rc<rudder::Type>, usize)>,
    next_register_offset: usize,

    /// Functions
    functions: HashMap<InternedString, (FunctionKind, Function, boom::FunctionDefinition)>,
}

impl BuildContext {
    fn add_register(&mut self, name: InternedString, typ: Rc<Type>) {
        self.registers
            .insert(name, (typ.clone(), self.next_register_offset));

        // 8 byte aligned
        self.next_register_offset += typ.width_bytes().next_multiple_of(8)
    }

    fn add_struct(&mut self, name: InternedString, fields: &[boom::NamedType]) {
        let typ = Rc::new(Type::Composite(
            fields
                .iter()
                .map(|boom::NamedType { typ, .. }| self.resolve_type(typ.clone()))
                .collect(),
        ));

        let fields = fields
            .iter()
            .enumerate()
            .map(|(idx, boom::NamedType { name, .. })| (*name, idx))
            .collect();

        if self.structs.insert(name, (typ, fields)).is_some() {
            panic!("struct with name {name} already added");
        }
    }

    fn add_union(&mut self, name: InternedString, fields: &[boom::NamedType]) {
        let typ = Rc::new(Type::Composite(
            fields
                .iter()
                .map(|boom::NamedType { typ, .. }| self.resolve_type(typ.clone()))
                .collect(),
        ));

        let union_fields = fields
            .iter()
            .enumerate()
            .map(|(idx, boom::NamedType { name, .. })| (*name, idx))
            .collect();

        if self
            .unions
            .insert(name, (typ.clone(), union_fields))
            .is_some()
        {
            panic!("union with name {name} already added");
        }
    }

    fn add_enum(&mut self, name: InternedString, variants: &[InternedString]) {
        let typ = Rc::new(Type::u32());

        let variants = variants
            .iter()
            .enumerate()
            .map(|(idx, name)| (*name, u32::try_from(idx).unwrap()))
            .collect();

        if self.enums.insert(name, (typ, variants)).is_some() {
            panic!("enum with name {name} already added");
        }
    }

    fn add_function(&mut self, name: InternedString, definition: &boom::FunctionDefinition) {
        self.functions.insert(
            name,
            (
                FunctionKind::Execute,
                rudder::Function::new(
                    name,
                    self.resolve_type(definition.signature.return_type.clone()),
                    definition.signature.parameters.borrow().iter().map(
                        |boom::Parameter { typ, name, is_ref }| {
                            assert!(!is_ref, "no reference parameters allowed");
                            (*name, self.resolve_type(typ.clone()))
                        },
                    ),
                ),
                definition.clone(),
            ),
        );
    }

    fn build_functions(&mut self) {
        self.functions
            .clone()
            .into_iter()
            .for_each(|(name, (_kind, rudder_fn, boom_fn))| {
                log::debug!("building function {name:?}");
                FunctionBuildContext::new(self, rudder_fn.clone()).build_fn(boom_fn.clone());
            });
    }

    fn resolve_type(&self, typ: Rc<RefCell<boom::Type>>) -> Rc<rudder::Type> {
        match &*typ.borrow() {
            boom::Type::Unit => Rc::new(rudder::Type::unit()),
            boom::Type::String => Rc::new(rudder::Type::u32()), // same as InternedString raw value
            boom::Type::Bool | boom::Type::Bit => Rc::new(rudder::Type::u1()),
            boom::Type::Real => Rc::new(rudder::Type::f32()),
            boom::Type::Float => Rc::new(rudder::Type::f64()),
            boom::Type::Int { signed, size } => {
                let tc = match signed {
                    true => rudder::PrimitiveTypeClass::SignedInteger,
                    false => rudder::PrimitiveTypeClass::UnsignedInteger,
                };

                let element_width = match size {
                    boom::Size::Static(size) => *size,
                    boom::Size::Runtime(_) | boom::Size::Unknown => panic!("BOOM bitvector monomorphisation pass should ensure all bitvectors have static length before rudder conversion"),
                };

                Rc::new(rudder::Type::new_primitive(tc, element_width))
            }
            boom::Type::Constant(_) => todo!(),
            boom::Type::Enum { name, .. } => self.enums.get(name).unwrap().0.clone(),
            boom::Type::Union { name, .. } => self.unions.get(name).unwrap().0.clone(),
            boom::Type::Struct { name, .. } => self.structs.get(name).unwrap().0.clone(),
            boom::Type::List { .. } => todo!(),
            boom::Type::Vector { element_type } => {
                let element_type = (*self.resolve_type(element_type.clone())).clone();
                // todo: Brian Campbell said the Sail C backend had functionality to staticize
                // all bitvector lengths
                Rc::new(element_type.vectorize(None))
            }
            boom::Type::FixedVector {
                length,
                element_type,
            } => {
                let element_type = (*self.resolve_type(element_type.clone())).clone();

                Rc::new(element_type.vectorize(Some(usize::try_from(*length).unwrap())))
            }
            boom::Type::Reference(inner) => {
                // todo: this is broken:(
                self.resolve_type(inner.clone())
            }
        }
    }

    fn finalise(self) -> Context {
        Context {
            fns: self
                .functions
                .into_iter()
                .map(|(name, (kind, f, _))| (name, (kind, f)))
                .collect(),
            structs: self.structs.into_iter().map(|(_, (typ, _))| typ).collect(),
            unions: self.unions.into_iter().map(|(_, (typ, _))| typ).collect(),
            registers: self.registers.into_values().collect(),
        }
    }
}

struct FunctionBuildContext<'ctx> {
    build_context: &'ctx mut BuildContext,
    rudder_fn: Function,
    blocks: HashMap<boom::control_flow::ControlFlowBlock, rudder::Block>,
}

impl<'ctx> FunctionBuildContext<'ctx> {
    pub fn new(build_context: &'ctx mut BuildContext, rudder_fn: Function) -> Self {
        Self {
            build_context,
            rudder_fn,
            blocks: HashMap::default(),
        }
    }

    pub fn resolve_block(
        &mut self,
        boom_block: boom::control_flow::ControlFlowBlock,
    ) -> rudder::Block {
        if let Some(block) = self.blocks.get(&boom_block) {
            block.clone()
        } else {
            let block = BlockBuildContext::new(self).build_block(boom_block.clone());
            self.blocks.insert(boom_block, block.clone());
            block
        }
    }

    pub fn build_fn(&mut self, boom_fn: boom::FunctionDefinition) {
        self.rudder_fn.inner.borrow_mut().entry_block = self.resolve_block(boom_fn.entry_block);
    }
}

struct BlockBuildContext<'ctx, 'fn_ctx> {
    function_build_context: &'fn_ctx mut FunctionBuildContext<'ctx>,
    statement_builder: StatementBuilder,
}

impl<'ctx: 'fn_ctx, 'fn_ctx> BlockBuildContext<'ctx, 'fn_ctx> {
    pub fn new(function_build_context: &'fn_ctx mut FunctionBuildContext<'ctx>) -> Self {
        Self {
            function_build_context,
            statement_builder: StatementBuilder::new(),
        }
    }

    fn ctx(&mut self) -> &mut BuildContext {
        self.function_build_context.build_context
    }

    fn fn_ctx(&mut self) -> &mut FunctionBuildContext<'ctx> {
        self.function_build_context
    }

    fn build_block(mut self, boom_block: boom::control_flow::ControlFlowBlock) -> rudder::Block {
        log::trace!("building block: {}", boom_block);

        // convert statements
        boom_block
            .statements()
            .iter()
            .for_each(|stmt| self.build_statement(stmt.clone()));

        // check terminator, insert final rudder statement
        let kind = match boom_block.terminator() {
            boom::control_flow::Terminator::Return(value) => {
                let value = value.map(|v| self.build_value(Rc::new(RefCell::new(v))));

                rudder::StatementKind::Return { value }
            }

            boom::control_flow::Terminator::Conditional {
                condition,
                target: boom_target,
                fallthrough: boom_fallthrough,
            } => {
                let condition = self.build_value(Rc::new(RefCell::new(condition)));
                let typ = condition.typ();

                if *typ != Type::u1() {
                    // so far this todo is never hit, but if you do hit it implement it pls
                    todo!("insert cast from {} to u1", condition.typ());
                }

                let rudder_true_target = self.fn_ctx().resolve_block(boom_target);
                let rudder_false_target = self.fn_ctx().resolve_block(boom_fallthrough);

                StatementKind::Branch {
                    condition,
                    true_target: rudder_true_target,
                    false_target: rudder_false_target,
                }
            }
            boom::control_flow::Terminator::Unconditional {
                target: boom_target,
            } => {
                let rudder_target = self.fn_ctx().resolve_block(boom_target);
                StatementKind::Jump {
                    target: rudder_target,
                }
            }
        };

        self.statement_builder.build(kind);

        let rudder_block = rudder::Block::new();
        rudder_block.set_statements(self.statement_builder.finish().into_iter());
        rudder_block
    }

    fn build_statement(&mut self, statement: Rc<RefCell<boom::Statement>>) {
        match &*statement.borrow() {
            boom::Statement::TypeDeclaration { name, typ } => {
                let typ = self.ctx().resolve_type(typ.clone());
                self.fn_ctx().rudder_fn.add_local_variable(*name, typ);
            }
            boom::Statement::Copy { expression, value } => {
                self.build_copy(value.clone(), expression);
            }
            boom::Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => {
                self.build_function_call(arguments, name, expression);
            }

            boom::Statement::Label(_)
            | boom::Statement::Goto(_)
            | boom::Statement::Jump { .. }
            | boom::Statement::End(_)
            | boom::Statement::Undefined
            | boom::Statement::If { .. } => {
                unreachable!("no control flow should exist at this point in compilation!")
            }
            boom::Statement::Exit(_) | boom::Statement::Comment(_) => (),
        }
    }

    fn build_copy(&mut self, value: Rc<RefCell<boom::Value>>, expression: &boom::Expression) {
        // ignore copies of undefined
        if let boom::Value::Literal(lit) = &*value.borrow() {
            if let boom::Literal::Undefined = *lit.borrow() {
                return;
            }
        }

        let source = self.build_value(value.clone());

        let source_type = source.typ();

        match expression {
            boom::Expression::Identifier(ident) => {
                match self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                    Some(symbol) => {
                        let value = if symbol.typ() != source_type {
                            self.generate_cast(source, source_type, symbol.typ())
                        } else {
                            source
                        };

                        self.statement_builder
                            .build(StatementKind::WriteVariable { symbol, value });
                    }
                    None => {
                        //register lookup
                        let Some((register_type, offset)) =
                            self.ctx().registers.get(ident).cloned()
                        else {
                            panic!("wtf is {ident}");
                        };

                        let value = if register_type != source_type {
                            self.generate_cast(source, source_type, register_type.clone())
                        } else {
                            source
                        };

                        let offset = self.statement_builder.build(StatementKind::Constant {
                            typ: Rc::new(Type::u32()),
                            value: rudder::ConstantValue::UnsignedInteger(offset),
                        });

                        self.statement_builder.build(StatementKind::WriteRegister {
                            offset: offset.clone(),
                            value,
                        });
                    }
                }
            }
            boom::Expression::Field { expression, field } => {
                // todo: insert casts if necessary here!!
                match &**expression {
                    boom::Expression::Identifier(ident) => {
                        // lookup identifier
                        if let Some(symbol) = self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                            // copying into local variable
                            let target_typ = symbol.typ();

                            let structs = self.ctx().structs.clone();
                            let (_, (struct_typ, fields)) = structs
                                .iter()
                                .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                                .expect("failed to find struct :(");

                            let idx = fields.get(field).unwrap();

                            // read variable
                            let read_var =
                                self.statement_builder.build(StatementKind::ReadVariable {
                                    symbol: symbol.clone(),
                                });

                            // cast to field type
                            let field_type = {
                                let Type::Composite(field_types) = &**struct_typ else {
                                    unreachable!();
                                };

                                field_types[*idx].clone()
                            };

                            let cast = self.statement_builder.build(StatementKind::Cast {
                                kind: CastOperationKind::Reinterpret,
                                typ: field_type,
                                value: source,
                            });

                            // modify field
                            let mutate_field =
                                self.statement_builder.build(StatementKind::MutateField {
                                    composite: read_var,
                                    field: *idx,
                                    value: cast,
                                });

                            // write variable
                            self.statement_builder.build(StatementKind::WriteVariable {
                                symbol,
                                value: mutate_field,
                            });

                            // todo: nesting, what's that?
                        } else if let Some((typ, offset)) = self.ctx().registers.get(ident).cloned()
                        {
                            // writing into composite register

                            let (idx, field_type) = {
                                let target_typ = typ.clone();

                                let (_, (struct_type, fields)) = self
                                    .ctx()
                                    .structs
                                    .iter()
                                    .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                                    .expect("failed to find struct :(");

                                let Type::Composite(field_types) = &**struct_type else {
                                    unreachable!();
                                };

                                let idx = *fields.get(field).unwrap();

                                (idx, field_types[idx].clone())
                            };

                            // push offset to statemetns
                            let offset_statement =
                                self.statement_builder.build(StatementKind::Constant {
                                    typ: Rc::new(Type::u64()),
                                    value: rudder::ConstantValue::UnsignedInteger(offset),
                                });

                            // read whole register
                            let read_reg =
                                self.statement_builder.build(StatementKind::ReadRegister {
                                    typ: typ.clone(),
                                    offset: offset_statement.clone(),
                                });

                            let cast = self.statement_builder.build(StatementKind::Cast {
                                kind: CastOperationKind::Reinterpret,
                                typ: field_type,
                                value: source,
                            });

                            // mutate field
                            let mutate_field =
                                self.statement_builder.build(StatementKind::MutateField {
                                    composite: read_reg,
                                    field: idx,
                                    value: cast,
                                });

                            // write whole register
                            self.statement_builder.build(StatementKind::WriteRegister {
                                offset: offset_statement,
                                value: mutate_field,
                            });
                        } else {
                            panic!("{ident} not local var or register");
                        };
                    }
                    boom::Expression::Field { .. } => todo!("nested field expressions"),
                    boom::Expression::Address(_) => todo!(),
                }
            }
            boom::Expression::Address(_) => todo!(),
        }
    }

    fn build_function_call(
        &mut self,
        arguments: &[Rc<RefCell<boom::Value>>],
        name: &InternedString,
        expression: &Option<boom::Expression>,
    ) {
        let args = arguments
            .iter()
            .map(|arg| self.build_value(arg.clone()))
            .collect::<Vec<_>>();

        let fn_statement = if let Some(fn_statement) = self.build_specialized_function(*name, &args)
        {
            fn_statement
        } else {
            let target = match self.ctx().functions.get(name).cloned() {
                Some((_, target, _)) => target,
                None => panic!("unknown function {name}"),
            };

            // cast all arguments to the correct type
            let casts = args
                .iter()
                .enumerate()
                .map(|(i, stmt)| {
                    let typ = target.signature().1[i].typ();
                    self.statement_builder.build(StatementKind::Cast {
                        kind: CastOperationKind::Reinterpret,
                        typ,
                        value: stmt.clone(),
                    })
                })
                .collect::<Vec<_>>();

            // call statement
            self.statement_builder.build(StatementKind::Call {
                target,
                args: casts.clone(),
            })
        };

        // copy to expr statement
        if let Some(expression) = expression {
            match expression {
                boom::Expression::Identifier(ident) => {
                    match self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                        Some(symbol) => {
                            let cast = self.statement_builder.build(StatementKind::Cast {
                                kind: CastOperationKind::Reinterpret,
                                typ: symbol.typ(),
                                value: fn_statement,
                            });

                            self.statement_builder.build(StatementKind::WriteVariable {
                                symbol,
                                value: cast,
                            });
                        }
                        None => {
                            //register lookup
                            let Some(reg) = self.ctx().registers.get(ident).cloned() else {
                                panic!("wtf is {ident}");
                            };
                            let offset = self.statement_builder.build(StatementKind::Constant {
                                typ: Rc::new(Type::u32()),
                                value: rudder::ConstantValue::UnsignedInteger(reg.1),
                            });
                            self.statement_builder.build(StatementKind::WriteRegister {
                                offset,
                                value: fn_statement,
                            });
                        }
                    }
                }
                boom::Expression::Field { .. } => todo!(),
                boom::Expression::Address(_) => todo!(),
            }
        }
    }

    /// Sail compiler builtin functions only!
    fn build_specialized_function(
        &mut self,
        name: InternedString,
        args: &[Statement],
    ) -> Option<Statement> {
        match name.as_ref() {
            "trap" => Some(self.statement_builder.build(StatementKind::Trap)),
            "%i64->%i" => Some(self.statement_builder.build(StatementKind::Cast {
                kind: CastOperationKind::ZeroExtend,
                typ: Rc::new(Type::u64()),
                value: args[0].clone(),
            })),
            "%i->%i64" => Some(self.statement_builder.build(StatementKind::Cast {
                kind: CastOperationKind::ZeroExtend,
                typ: Rc::new(Type::u64()),
                value: args[0].clone(),
            })),
            "subrange_bits" => {
                // end - start + 1
                let one = self.statement_builder.build(StatementKind::Constant {
                    typ: args[1].typ(),
                    value: rudder::ConstantValue::SignedInteger(1),
                });
                let diff = self
                    .statement_builder
                    .build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Sub,
                        lhs: args[1].clone(),
                        rhs: args[2].clone(),
                    });
                let len = self
                    .statement_builder
                    .build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: diff.clone(),
                        rhs: one.clone(),
                    });

                Some(self.statement_builder.build(StatementKind::BitExtract {
                    value: args[0].clone(),
                    start: args[2].clone(),
                    length: len,
                }))
            }
            "eq_bits" => Some(
                self.statement_builder
                    .build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::CmpEq,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    }),
            ),
            "slice" => {
                // uint64 n, uint64 start, uint64 len
                Some(self.statement_builder.build(StatementKind::BitExtract {
                    value: args[0].clone(),
                    start: args[1].clone(),
                    length: args[2].clone(),
                }))
            }
            // todo: should probably be casts
            "UInt0" => Some(args[0].clone()),
            "SInt0" => Some(args[0].clone()),

            "ZeroExtend0" => {
                let value = args[0].clone();
                let _length = args[1].clone();
                Some(self.statement_builder.build(StatementKind::Cast {
                    kind: CastOperationKind::ZeroExtend,
                    typ: Rc::new(Type::u64()),
                    value,
                }))
            }
            "read_gpr_from_vector" => Some(self.statement_builder.build(StatementKind::Constant {
                typ: Rc::new(Type::u64()),
                value: rudder::ConstantValue::UnsignedInteger(0),
            })),
            "get_slice_int" => Some(self.statement_builder.build(StatementKind::BitExtract {
                value: args[1].clone(),
                start: args[2].clone(),
                length: args[0].clone(),
            })),
            "bitvector_access" => {
                let length = self.statement_builder.build(StatementKind::Constant {
                    typ: Rc::new(Type::u64()),
                    value: rudder::ConstantValue::UnsignedInteger(1),
                });
                let bitex = self.statement_builder.build(StatementKind::BitExtract {
                    value: args[0].clone(),
                    start: args[1].clone(),
                    length,
                });

                Some(self.statement_builder.build(StatementKind::Cast {
                    kind: CastOperationKind::Truncate,
                    typ: Rc::new(Type::u1()),
                    value: bitex,
                }))
            }
            "make_the_value" => Some(args[0].clone()),
            "size_itself_int" => Some(args[0].clone()),
            "truncate" => Some(self.statement_builder.build(StatementKind::Cast {
                kind: CastOperationKind::Truncate,
                typ: Rc::new(Type::new_primitive(
                    rudder::PrimitiveTypeClass::UnsignedInteger,
                    8,
                )),
                value: args[0].clone(),
            })),
            _ => None,
        }
        // if name.as_ref() == "trap" {
        //     Some(self.statement_builder.build(StatementKind::Trap))
        // } else if name.as_ref() == "read_pc" {
        //     Some(self.statement_builder.build(StatementKind::ReadPc))
        // } else if name.as_ref() == "vector_subrange_A" {
        //     // end - start + 1
        //     let one = self.statement_builder.build(StatementKind::Constant {
        //         typ: args[1].typ(),
        //         value: rudder::ConstantValue::SignedInteger(1),
        //     });
        //     let diff = self
        //         .statement_builder
        //         .build(StatementKind::BinaryOperation {
        //             kind: BinaryOperationKind::Sub,
        //             lhs: args[2].clone(),
        //             rhs: args[1].clone(),
        //         });
        //     let len = self
        //         .statement_builder
        //         .build(StatementKind::BinaryOperation {
        //             kind: BinaryOperationKind::Add,
        //             lhs: diff.clone(),
        //             rhs: one.clone(),
        //         });

        //     Some(self.statement_builder.build(StatementKind::BitExtract {
        //         value: args[0].clone(),
        //         start: args[1].clone(),
        //         length: len,
        //     }))
        // } else if name.as_ref() == "slice" {

        // } else if name.as_ref().starts_with("vector_access_A") {
        //     Some(self.statement_builder.build(StatementKind::ReadElement {
        //         vector: args[0].clone(),
        //         index: args[1].clone(),
        //     }))
        // } else if name.as_ref().starts_with("vector_update_B") {
        //     Some(self.statement_builder.build(StatementKind::MutateElement {
        //         vector: args[0].clone(),
        //         value: args[2].clone(),
        //         index: args[1].clone(),
        //     }))
        // } else if name.as_ref() == "u__raw_GetSlice_int" {
        //     // u__raw_GetSlice_int(len, n, start)
        //     Some(self.statement_builder.build(StatementKind::BitExtract {
        //         value: args[1].clone(),
        //         start: args[2].clone(),
        //         length: args[0].clone(),
        //     }))
        // } else if name.as_ref() == "ZeroExtend__0" {
        //     let value = args[0].clone();
        //     let _length = args[1].clone();
        //     Some(self.statement_builder.build(StatementKind::Cast {
        //         kind: CastOperationKind::ZeroExtend,
        //         typ: Rc::new(Type::u64()),
        //         value,
        //     }))
        // } else if name.as_ref() == "%i64->%i" {
        //     let value = args[0].clone();
        //     Some(self.statement_builder.build(StatementKind::Cast {
        //         kind: CastOperationKind::ZeroExtend,
        //         typ: Rc::new(Type::u64()),
        //         value,
        //     }))
        // } else {
        //     None
        // }
    }

    /// Last statement returned is the value
    fn build_value(&mut self, boom_value: Rc<RefCell<boom::Value>>) -> Statement {
        match &*boom_value.borrow() {
            boom::Value::Identifier(ident) => {
                if let Some(symbol) = self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                    return self
                        .statement_builder
                        .build(StatementKind::ReadVariable { symbol });
                }

                if let Some(symbol) = self.fn_ctx().rudder_fn.get_parameter(*ident) {
                    return self
                        .statement_builder
                        .build(StatementKind::ReadVariable { symbol });
                }

                if let Some((typ, offset)) = self.ctx().registers.get(ident).cloned() {
                    let offset = self.statement_builder.build(StatementKind::Constant {
                        typ: Rc::new(Type::u32()),
                        value: rudder::ConstantValue::UnsignedInteger(offset),
                    });

                    return self.statement_builder.build(StatementKind::ReadRegister {
                        typ: typ.clone(),
                        offset,
                    });
                }

                if let Some(value) = self
                    .ctx()
                    .enums
                    .iter()
                    .find_map(|(_, (_, variants))| variants.get(ident))
                    .cloned()
                {
                    return self.statement_builder.build(StatementKind::Constant {
                        typ: Rc::new(Type::u32()),
                        value: rudder::ConstantValue::UnsignedInteger(value.try_into().unwrap()),
                    });
                }

                panic!("unknown ident: {:?}\n{:?}", ident, boom_value);
            }
            boom::Value::Literal(literal) => {
                let kind = match &*literal.borrow() {
                    boom::Literal::Int(i) => StatementKind::Constant {
                        typ: Rc::new(Type::u64()),
                        value: rudder::ConstantValue::SignedInteger(i.try_into().unwrap()),
                    },
                    boom::Literal::Bits(_bits) => todo!(),
                    boom::Literal::Bit(bit) => StatementKind::Constant {
                        typ: Rc::new(Type::u1()),
                        value: rudder::ConstantValue::UnsignedInteger(
                            bit.value().try_into().unwrap(),
                        ),
                    },
                    boom::Literal::Bool(b) => StatementKind::Constant {
                        typ: Rc::new(Type::u1()),
                        value: rudder::ConstantValue::UnsignedInteger(if *b { 1 } else { 0 }),
                    },
                    boom::Literal::String(str) => StatementKind::Constant {
                        typ: Rc::new(Type::u32()),
                        value: rudder::ConstantValue::UnsignedInteger(
                            str.key().try_into().unwrap(),
                        ),
                    },
                    boom::Literal::Unit => StatementKind::Constant {
                        typ: Rc::new(Type::unit()),
                        value: rudder::ConstantValue::Unit,
                    },
                    boom::Literal::Reference(_) => todo!(),
                    boom::Literal::Undefined => unimplemented!(),
                };

                self.statement_builder.build(kind)
            }
            boom::Value::Operation(op) => self.build_operation(op),
            boom::Value::Struct { name, fields } => {
                let (typ, field_name_index_map) = self.ctx().structs.get(name).cloned().unwrap();

                let mut field_statements = vec![None; fields.len()];

                for NamedValue { name, value } in fields {
                    let field_statement = self.build_value(value.clone());
                    let idx = field_name_index_map.get(name).unwrap();
                    field_statements[*idx] = Some(field_statement);
                }

                self.statement_builder
                    .build(StatementKind::CreateComposite {
                        typ: typ.clone(),
                        fields: field_statements.into_iter().map(|o| o.unwrap()).collect(),
                    })
            }
            boom::Value::Field { value, field_name } => {
                let ident = match &*value.borrow() {
                    boom::Value::Identifier(ident) => *ident,
                    _ => todo!(),
                };

                self.build_value(value.clone());

                // lookup identifier
                if let Some(symbol) = self.fn_ctx().rudder_fn.get_local_variable(ident) {
                    // copying into local variable
                    let target_typ = symbol.typ();

                    let structs = self.ctx().structs.clone();
                    let (_, (_, fields)) = structs
                        .iter()
                        .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                        .expect("failed to find struct :(");

                    let idx = fields.get(field_name).unwrap();

                    let read_var = self
                        .statement_builder
                        .build(StatementKind::ReadVariable { symbol });

                    self.statement_builder.build(StatementKind::ReadField {
                        composite: read_var.clone(),
                        field: *idx,
                    })
                } else if let Some((typ, reg_offset)) = self.ctx().registers.get(&ident).cloned() {
                    // writing into composite register

                    let offset_statement = self.statement_builder.build(StatementKind::Constant {
                        typ: Rc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(reg_offset),
                    });

                    let read_reg = self.statement_builder.build(StatementKind::ReadRegister {
                        typ: typ.clone(),
                        offset: offset_statement,
                    });

                    let target_typ = typ.clone();
                    let structs = self.ctx().structs.clone();
                    let (_, (_, fields)) = structs
                        .iter()
                        .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                        .expect("failed to find struct :(");

                    let idx = fields.get(field_name).unwrap();

                    self.statement_builder.build(StatementKind::ReadField {
                        composite: read_reg,
                        field: *idx,
                    })
                } else {
                    panic!("{ident} not local var or register");
                }

                // if value is register, find reg name + offset emit
                // read_register if value is local variable,
                // emit read_variable with Some(offset)
            }
            boom::Value::CtorKind { .. } => todo!(),
            boom::Value::CtorUnwrap { .. } => todo!(),
        }
    }

    fn build_operation(&mut self, op: &boom::Operation) -> Statement {
        match op {
            boom::Operation::Not(value) => {
                let value = self.build_value(value.clone());
                self.statement_builder.build(StatementKind::UnaryOperation {
                    kind: rudder::UnaryOperationKind::Not,
                    value,
                })
            }
            boom::Operation::Complement(value) => {
                let value = self.build_value(value.clone());
                self.statement_builder.build(StatementKind::UnaryOperation {
                    kind: rudder::UnaryOperationKind::Complement,
                    value,
                })
            }
            boom::Operation::Cast(value, typ) => {
                let target_type = self.ctx().resolve_type(typ.clone());
                let value = self.build_value(value.clone());

                let source_type = value.typ();

                let kind = match *source_type {
                    Type::Composite(_) | Type::Vector { .. } => {
                        panic!("cast on non-primitive type")
                    }
                    Type::Primitive(_) => {
                        match source_type.width_bits().cmp(&target_type.width_bits()) {
                            Ordering::Less => CastOperationKind::ZeroExtend,
                            Ordering::Greater => CastOperationKind::Truncate,
                            Ordering::Equal => CastOperationKind::Reinterpret,
                        }
                    }
                };

                self.statement_builder.build(StatementKind::Cast {
                    kind,
                    typ: target_type,
                    value,
                })
            }

            boom::Operation::LeftShift(value, amount)
            | boom::Operation::RightShift(value, amount)
            | boom::Operation::RotateRight(value, amount)
            | boom::Operation::RotateLeft(value, amount) => {
                let value = self.build_value(value.clone());
                let amount = self.build_value(amount.clone());

                let kind = match op {
                    boom::Operation::LeftShift(_, _) => ShiftOperationKind::LogicalShiftLeft,
                    boom::Operation::RightShift(_, _) => {
                        // todo figure out if logical or arithmetic
                        ShiftOperationKind::LogicalShiftRight
                    }
                    boom::Operation::RotateRight(_, _) => ShiftOperationKind::RotateRight,
                    boom::Operation::RotateLeft(_, _) => ShiftOperationKind::RotateLeft,

                    _ => unreachable!(),
                };

                self.statement_builder.build(StatementKind::ShiftOperation {
                    kind,
                    value,
                    amount,
                })
            }

            boom::Operation::Equal(left, right)
            | boom::Operation::NotEqual(left, right)
            | boom::Operation::LessThan(left, right)
            | boom::Operation::LessThanOrEqual(left, right)
            | boom::Operation::GreaterThan(left, right)
            | boom::Operation::GreaterThanOrEqual(left, right)
            | boom::Operation::Subtract(left, right)
            | boom::Operation::Add(left, right)
            | boom::Operation::Or(left, right)
            | boom::Operation::Multiply(left, right)
            | boom::Operation::And(left, right)
            | boom::Operation::Xor(left, right)
            | boom::Operation::Divide(left, right) => {
                let mut lhs = self.build_value(left.clone());
                let mut rhs = self.build_value(right.clone());

                if lhs.typ() != rhs.typ() {
                    // need to insert casts
                    let destination_type = if lhs.typ().width_bits() > rhs.typ().width_bits() {
                        lhs.typ()
                    } else {
                        rhs.typ()
                    };

                    lhs = self.generate_cast(lhs.clone(), lhs.typ(), destination_type.clone());
                    rhs = self.generate_cast(rhs.clone(), rhs.typ(), destination_type);
                }

                self.statement_builder
                    .build(StatementKind::BinaryOperation {
                        kind: match op {
                            boom::Operation::Equal(_, _) => BinaryOperationKind::CmpEq,
                            boom::Operation::NotEqual(_, _) => BinaryOperationKind::CmpNe,
                            boom::Operation::LessThan(_, _) => BinaryOperationKind::CmpLt,
                            boom::Operation::LessThanOrEqual(_, _) => BinaryOperationKind::CmpLe,
                            boom::Operation::GreaterThan(_, _) => BinaryOperationKind::CmpGt,
                            boom::Operation::GreaterThanOrEqual(_, _) => BinaryOperationKind::CmpGe,
                            boom::Operation::Subtract(_, _) => BinaryOperationKind::Sub,
                            boom::Operation::Add(_, _) => BinaryOperationKind::Add,
                            boom::Operation::Or(_, _) => BinaryOperationKind::Or,
                            boom::Operation::Multiply(_, _) => BinaryOperationKind::Multiply,
                            boom::Operation::And(_, _) => BinaryOperationKind::And,
                            boom::Operation::Xor(_, _) => BinaryOperationKind::Xor,
                            boom::Operation::Divide(_, _) => BinaryOperationKind::Divide,

                            _ => unreachable!(),
                        },
                        lhs,
                        rhs,
                    })
            }
        }
    }
    fn generate_cast(
        &mut self,
        source: Statement,
        _source_type: Rc<Type>,
        destination_type: Rc<Type>,
    ) -> Statement {
        // todo: do correct kind of cast for different source/dest pairs
        self.statement_builder.build(StatementKind::Cast {
            kind: CastOperationKind::Reinterpret,
            typ: destination_type,
            value: source,
        })
    }
}

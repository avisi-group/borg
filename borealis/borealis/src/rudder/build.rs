use {
    crate::{
        boom,
        rudder::{
            self, BinaryOperationKind, Block, CastOperationKind, Context, Function, FunctionInner,
            FunctionKind, ShiftOperationKind, Statement, StatementKind, Symbol, Type,
        },
    },
    common::{intern::InternedString, HashMap},
    log::warn,
    std::{cell::RefCell, cmp::Ordering, rc::Rc},
};

pub fn from_boom(ast: &boom::Ast) -> Context {
    let mut build_ctx = BuildContext::default();

    // DEFINITION ORDER DEPENDANT!!!
    ast.definitions.iter().for_each(|def| match def {
        boom::Definition::Enum { name, variants } => build_ctx.add_enum(*name, variants),
        boom::Definition::Union { name, fields } => build_ctx.add_union(*name, fields),
        boom::Definition::Struct { name, fields } => build_ctx.add_struct(*name, fields),
        // todo
        boom::Definition::Pragma { .. } => (),
        boom::Definition::Let { .. } => (),
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
    composite_registers: HashMap<InternedString, Rc<rudder::Type>>,
    registers: HashMap<InternedString, (Rc<rudder::Type>, usize)>,
    next_register_offset: usize,

    /// Functions
    functions: HashMap<InternedString, (FunctionKind, Function, boom::FunctionDefinition)>,
}

impl BuildContext {
    fn add_register(&mut self, name: InternedString, typ: Rc<Type>) {
        match &*typ {
            Type::Primitive(_) => {
                self.registers
                    .insert(name, (typ.clone(), self.next_register_offset));

                self.next_register_offset += typ.width_bytes();
            }
            Type::Composite(types) => {
                self.composite_registers.insert(name, typ.clone());

                types.iter().enumerate().for_each(|(i, typ)| {
                    self.add_register(format!("{name}_{i}").into(), typ.clone())
                })
            }
            Type::Vector {
                element_count,
                element_type,
            } => {
                self.composite_registers.insert(name, typ.clone());

                for i in 0..*element_count {
                    self.add_register(format!("{name}_{i}").into(), element_type.clone());
                }
            }
        }
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

        // for field in fields {
        //     let field_type = self.resolve_type(field.typ.clone());
        //     if self
        //         .functions
        //         .insert(
        //             field.name,
        //             (
        //                 FunctionKind::Other,
        //                 Function::new(
        //                     field.name,
        //                     typ.clone(),
        //                     [(field.name, field_type)].into_iter(),
        //                 ),
        //                 boom::FunctionDefinition {
        //                     signature: FunctionSignature {
        //                         name: "IDONOTEXIST".into(),
        //                         parameters: Rc::new(RefCell::new(vec![])),
        //                         return_type:
        // Rc::new(RefCell::new(boom::Type::Unit)),
        // },                     entry_block:
        // boom::control_flow::ControlFlowBlock::new(),
        // },             ),
        //         )
        //         .is_some()
        //     {
        //         panic!("union constructor with name {} already exists",
        // field.name)     }
        // }
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
                    boom::Size::Runtime(_) | boom::Size::Unknown => 64, /* todo blame tom when this inevitably breaks */
                };

                Rc::new(rudder::Type::new_primitive(tc, element_width))
            }
            boom::Type::Constant(_) => todo!(),
            boom::Type::Enum { name, .. } => self.enums.get(name).unwrap().0.clone(),
            boom::Type::Union { name, .. } => self.unions.get(name).unwrap().0.clone(),
            boom::Type::Struct { name, .. } => self.structs.get(name).unwrap().0.clone(),
            boom::Type::List { .. } => todo!(),
            boom::Type::Vector { .. } => todo!(),
            boom::Type::FixedVector {
                length,
                element_type,
            } => {
                let element_type = (*self.resolve_type(element_type.clone())).clone();

                Rc::new(element_type.vectorize(usize::try_from(*length).unwrap()))
            }
            boom::Type::Reference(_) => todo!(),
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
}

impl<'ctx: 'fn_ctx, 'fn_ctx> BlockBuildContext<'ctx, 'fn_ctx> {
    pub fn new(function_build_context: &'fn_ctx mut FunctionBuildContext<'ctx>) -> Self {
        Self {
            function_build_context,
        }
    }

    fn ctx(&mut self) -> &mut BuildContext {
        self.function_build_context.build_context
    }

    fn fn_ctx(&mut self) -> &mut FunctionBuildContext<'ctx> {
        self.function_build_context
    }

    fn build_block(&mut self, boom_block: boom::control_flow::ControlFlowBlock) -> rudder::Block {
        log::trace!("building block: {}", boom_block);
        let rudder_block = rudder::Block::new();

        // convert statements
        rudder_block.set_statements(
            boom_block
                .statements()
                .into_iter()
                .flat_map(|stmt| self.build_statement(stmt)),
        );

        // check terminator, insert final rudder statement
        let kind = match boom_block.terminator() {
            boom::control_flow::Terminator::Return(value) => {
                let value = value.map(|v| {
                    let stmts = self.transform_value(Rc::new(RefCell::new(v)));
                    let value = stmts.last().unwrap().clone();
                    rudder_block.extend_statements(stmts.into_iter());
                    value
                });

                rudder::StatementKind::Return { value }
            }

            boom::control_flow::Terminator::Conditional {
                condition,
                target: boom_target,
                fallthrough: boom_fallthrough,
            } => {
                let stmts = self.transform_value(Rc::new(RefCell::new(condition)));
                let condition = stmts.last().unwrap().clone();
                let typ = condition.get_type();

                if *typ != Type::u1() {
                    // so far this todo is never hit, but if you do hit it implement it pls
                    todo!("insert cast from {} to u1", condition.get_type());
                }

                rudder_block.extend_statements(stmts.into_iter());

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

        rudder_block.extend_statements([Statement::from_kind(kind)].into_iter());

        rudder_block
    }

    /// Last statement returned is the value
    fn transform_value(&mut self, boom_value: Rc<RefCell<boom::Value>>) -> Vec<Statement> {
        match &*boom_value.borrow() {
            boom::Value::Identifier(ident) => {
                if let Some(symbol) = self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                    return vec![Statement::from_kind(StatementKind::ReadVariable {
                        symbol,
                        member: None,
                    })];
                }

                if let Some(symbol) = self.fn_ctx().rudder_fn.get_parameter(*ident) {
                    return vec![Statement::from_kind(StatementKind::ReadVariable {
                        symbol,
                        member: None,
                    })];
                }

                if let Some((typ, offset)) = self.ctx().registers.get(ident) {
                    let offset_statement = Statement::from_kind(StatementKind::Constant {
                        typ: Rc::new(Type::u32()),
                        value: rudder::ConstantValue::UnsignedInteger(*offset),
                    });
                    return vec![
                        offset_statement.clone(),
                        Statement::from_kind(StatementKind::ReadRegister {
                            typ: typ.clone(),
                            offset: offset_statement,
                        }),
                    ];
                }

                if let Some(value) = self
                    .ctx()
                    .enums
                    .iter()
                    .find_map(|(_, (_, variants))| variants.get(ident))
                {
                    return vec![Statement::from_kind(StatementKind::Constant {
                        typ: Rc::new(Type::u32()),
                        value: rudder::ConstantValue::UnsignedInteger((*value).try_into().unwrap()),
                    })];
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
                };

                vec![Statement::from_kind(kind)]
            }
            boom::Value::Operation(op) => match op {
                boom::Operation::Not(value) => {
                    let mut stmts = self.transform_value(value.clone());
                    let op = Statement::from_kind(StatementKind::UnaryOperation {
                        kind: rudder::UnaryOperationKind::Not,
                        value: stmts.last().unwrap().clone(),
                    });

                    stmts.push(op);
                    stmts
                }
                boom::Operation::Complement(value) => {
                    let mut stmts = self.transform_value(value.clone());
                    let op = Statement::from_kind(StatementKind::UnaryOperation {
                        kind: rudder::UnaryOperationKind::Complement,
                        value: stmts.last().unwrap().clone(),
                    });

                    stmts.push(op);
                    stmts
                }
                boom::Operation::Cast(value, typ) => {
                    let target_type = self.ctx().resolve_type(typ.clone());
                    let mut stmts = self.transform_value(value.clone());
                    let value = stmts.last().unwrap().clone();

                    let source_type = value.get_type();

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

                    let statement = Statement::from_kind(StatementKind::Cast {
                        kind,
                        typ: target_type,
                        value,
                    });

                    stmts.push(statement);
                    stmts
                }

                boom::Operation::LeftShift(value, amount)
                | boom::Operation::RightShift(value, amount)
                | boom::Operation::RotateRight(value, amount)
                | boom::Operation::RotateLeft(value, amount) => {
                    let value_statements = self.transform_value(value.clone());
                    let amount_statements = self.transform_value(amount.clone());

                    let value = value_statements.last().unwrap().clone();
                    let amount = amount_statements.last().unwrap().clone();

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

                    let statement = Statement::from_kind(StatementKind::ShiftOperation {
                        kind,
                        value,
                        amount,
                    });

                    value_statements
                        .into_iter()
                        .chain(amount_statements)
                        .chain([statement])
                        .collect()
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
                    let mut left_statements = self.transform_value(left.clone());
                    let mut right_statements = self.transform_value(right.clone());

                    let mut lhs = left_statements.last().unwrap().clone();
                    let mut rhs = right_statements.last().unwrap().clone();

                    if lhs.get_type() != rhs.get_type() {
                        // need to insert casts
                        let destination_type =
                            if lhs.get_type().width_bits() > rhs.get_type().width_bits() {
                                lhs.get_type()
                            } else {
                                rhs.get_type()
                            };

                        lhs = {
                            let cast = generate_cast(
                                lhs.clone(),
                                lhs.get_type(),
                                destination_type.clone(),
                            );
                            left_statements.push(cast.clone());
                            cast
                        };

                        rhs = {
                            let cast = generate_cast(rhs.clone(), rhs.get_type(), destination_type);
                            right_statements.push(cast.clone());
                            cast
                        };
                    }

                    let statement = Statement::from_kind(StatementKind::BinaryOperation {
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
                    });

                    left_statements
                        .into_iter()
                        .chain(right_statements)
                        .chain([statement])
                        .collect()
                }
            },
            boom::Value::Struct { .. } => todo!(),
            boom::Value::Field { value, field_name } => {
                let ident = match &*value.borrow() {
                    boom::Value::Identifier(ident) => *ident,
                    _ => todo!(),
                };

                let mut stmts = self.transform_value(value.clone());

                // lookup identifier
                if let Some(symbol) = self.fn_ctx().rudder_fn.get_local_variable(ident) {
                    // copying into local variable
                    let target_typ = symbol.typ();

                    let (_, (_, fields)) = self
                        .ctx()
                        .structs
                        .iter()
                        .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                        .expect("failed to find struct :(");

                    let idx = fields.get(field_name).unwrap();
                    let statement = Statement::from_kind(StatementKind::ReadVariable {
                        symbol,
                        member: Some(*idx),
                    });
                    stmts.push(statement);
                } else if let Some(typ) = self.ctx().composite_registers.get(&ident) {
                    // writing into composite register
                    let target_typ = typ.clone();

                    let (struct_name, (_, fields)) = self
                        .ctx()
                        .structs
                        .iter()
                        .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                        .expect("failed to find struct :(");

                    let idx = fields.get(field_name).unwrap();

                    let register_name = InternedString::from(format!("{struct_name}_{idx}"));
                    let (register_type, offset) = self.ctx().registers.get(&register_name).unwrap();

                    let offset_statement = Statement::from_kind(StatementKind::Constant {
                        typ: Rc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(*offset),
                    });
                    stmts.push(offset_statement.clone());

                    stmts.push(Statement::from_kind(StatementKind::ReadRegister {
                        typ: register_type.clone(),
                        offset: offset_statement,
                    }));
                } else {
                    panic!("{ident} not local var or register");
                };

                // if value is register, find reg name + offset emit read_register
                // if value is local variable, emit read_variable with Some(offset)

                stmts
            }
            boom::Value::CtorKind { .. } => todo!(),
            boom::Value::CtorUnwrap { .. } => todo!(),
        }
    }

    fn build_statement(&mut self, statement: Rc<RefCell<boom::Statement>>) -> Vec<Statement> {
        match &*statement.borrow() {
            boom::Statement::TypeDeclaration { name, typ } => {
                let typ = self.ctx().resolve_type(typ.clone());
                self.fn_ctx().rudder_fn.add_local_variable(*name, typ);
                vec![]
            }
            boom::Statement::Copy { expression, value } => {
                let mut stmts = self.transform_value(value.clone());

                let source = stmts.last().unwrap().clone();
                let source_type = source.get_type();

                match expression {
                    boom::Expression::Identifier(ident) => {
                        match self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                            Some(symbol) => {
                                let value = if symbol.typ() != source_type {
                                    let cast = generate_cast(source, source_type, symbol.typ());
                                    stmts.push(cast.clone());
                                    cast
                                } else {
                                    source
                                };

                                let statement =
                                    Statement::from_kind(StatementKind::WriteVariable {
                                        symbol,
                                        value,
                                        member: None,
                                    });
                                stmts.push(statement);
                            }
                            None => {
                                //register lookup
                                let Some((register_type, offset)) = self.ctx().registers.get(ident)
                                else {
                                    panic!("wtf is {ident}");
                                };

                                let value = if **register_type != *source_type {
                                    let cast =
                                        generate_cast(source, source_type, register_type.clone());
                                    stmts.push(cast.clone());
                                    cast
                                } else {
                                    source
                                };

                                let offset = Statement::from_kind(StatementKind::Constant {
                                    typ: Rc::new(Type::u32()),
                                    value: rudder::ConstantValue::UnsignedInteger(*offset),
                                });
                                let statement =
                                    Statement::from_kind(StatementKind::WriteRegister {
                                        offset: offset.clone(),
                                        value,
                                    });
                                stmts.push(offset);
                                stmts.push(statement);
                            }
                        }
                    }
                    boom::Expression::Field { expression, field } => {
                        // todo: insert casts if necessary here!!
                        match &**expression {
                            boom::Expression::Identifier(ident) => {
                                // lookup identifier
                                if let Some(symbol) =
                                    self.fn_ctx().rudder_fn.get_local_variable(*ident)
                                {
                                    // copying into local variable
                                    let target_typ = symbol.typ();

                                    let (_, (_, fields)) = self
                                        .ctx()
                                        .structs
                                        .iter()
                                        .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                                        .expect("failed to find struct :(");

                                    let idx = fields.get(field).unwrap();
                                    let statement =
                                        Statement::from_kind(StatementKind::WriteVariable {
                                            symbol,
                                            value: stmts.last().unwrap().clone(),
                                            member: Some(*idx),
                                        });
                                    stmts.push(statement);
                                } else if let Some(typ) = self.ctx().composite_registers.get(ident)
                                {
                                    // writing into composite register
                                    let target_typ = typ.clone();

                                    let (_, (_, fields)) = self
                                        .ctx()
                                        .structs
                                        .iter()
                                        .find(|(_, (typ, _))| Rc::ptr_eq(&target_typ, typ))
                                        .expect("failed to find struct :(");

                                    let idx = fields.get(field).unwrap();

                                    let register_name =
                                        InternedString::from(format!("{ident}_{idx}"));
                                    let Some((_, offset)) =
                                        self.ctx().registers.get(&register_name)
                                    else {
                                        for (name, (typ, offset)) in &self.ctx().registers {
                                            println!("    {name}: {typ} @ {offset:x}");
                                        }
                                        panic!("register {register_name} not found",);
                                    };

                                    let offset_statement =
                                        Statement::from_kind(StatementKind::Constant {
                                            typ: Rc::new(Type::u64()),
                                            value: rudder::ConstantValue::UnsignedInteger(*offset),
                                        });
                                    stmts.push(offset_statement.clone());

                                    stmts.push(Statement::from_kind(
                                        StatementKind::WriteRegister {
                                            offset: offset_statement,
                                            value: stmts.last().unwrap().clone(),
                                        },
                                    ));
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

                stmts
            }
            boom::Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => {
                let (args, stmts): (Vec<Statement>, Vec<Vec<Statement>>) = arguments
                    .iter()
                    .map(|arg| self.transform_value(arg.clone()))
                    .map(|stmts| (stmts.last().unwrap().clone(), stmts))
                    .unzip();

                let stmts = stmts.into_iter().flatten().collect::<Vec<_>>();

                if *name == InternedString::from_static("trap") {
                    return vec![Statement::from_kind(StatementKind::Trap)];
                }

                if *name == InternedString::from_static("read_pc") {
                    return vec![Statement::from_kind(StatementKind::ReadPc)];
                }

                if *name == InternedString::from_static("vector_subrange_A") {
                    // end - start + 1
                    let one = Statement::from_kind(StatementKind::Constant {
                        typ: args[1].get_type(),
                        value: rudder::ConstantValue::SignedInteger(1),
                    });
                    let diff = Statement::from_kind(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Sub,
                        lhs: args[2].clone(),
                        rhs: args[1].clone(),
                    });
                    let len = Statement::from_kind(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: diff.clone(),
                        rhs: one.clone(),
                    });

                    return stmts
                        .into_iter()
                        .chain([
                            one,
                            diff,
                            len.clone(),
                            Statement::from_kind(StatementKind::BitExtract {
                                value: args[0].clone(),
                                start: args[1].clone(),
                                length: len,
                            }),
                        ])
                        .collect();
                }

                if *name == InternedString::from_static("slice") {
                    // uint64 n, uint64 start, uint64 len

                    return stmts
                        .into_iter()
                        .chain([Statement::from_kind(StatementKind::BitExtract {
                            value: args[0].clone(),
                            start: args[1].clone(),
                            length: args[2].clone(),
                        })])
                        .collect();
                }

                if *name == InternedString::from_static("vector_subrange_A") {}

                let target = match self.ctx().functions.get(name).cloned() {
                    Some((_, target, _)) => target,
                    None => {
                        // do this so that unimplemented has the correct type
                        let crate::boom::Expression::Identifier(ident) =
                            expression.as_ref().unwrap()
                        else {
                            panic!();
                        };
                        let typ = self
                            .fn_ctx()
                            .rudder_fn
                            .get_local_variable(*ident)
                            .unwrap()
                            .typ();

                        warn!("unknown function {name}");
                        Function {
                            inner: Rc::new(RefCell::new(FunctionInner {
                                name: format!("unimplemented_{name}").into(),
                                return_type: typ,
                                parameters: vec![
                                    Symbol {
                                        name: "removed".into(),
                                        kind: rudder::SymbolKind::Parameter,
                                        typ: Rc::new(rudder::Type::u64()),
                                    };
                                    64// todo: this is terrible, needed so that the casts on line `-9(%rip)` don't index out of bounds
                                ],
                                local_variables: HashMap::default(),
                                entry_block: Block::new(),
                            })),
                        }
                    }
                };

                // cast all arguments to the correct type
                let casts = args
                    .iter()
                    .enumerate()
                    .map(|(i, stmt)| {
                        let typ = target.signature().1[i].typ();
                        Statement::from_kind(StatementKind::Cast {
                            kind: CastOperationKind::Reinterpret,
                            typ,
                            value: stmt.clone(),
                        })
                    })
                    .collect::<Vec<_>>();

                // call statement
                let call = Statement::from_kind(StatementKind::Call {
                    target,
                    args: casts.clone(),
                });

                // copy to expr statement
                let copy = if let Some(expression) = expression {
                    match expression {
                        boom::Expression::Identifier(ident) => {
                            match self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                                Some(symbol) => {
                                    let statement =
                                        Statement::from_kind(StatementKind::WriteVariable {
                                            symbol,
                                            value: call.clone(),
                                            member: None,
                                        });
                                    vec![statement]
                                }
                                None => {
                                    //register lookup
                                    let Some(reg) = self.ctx().registers.get(ident) else {
                                        panic!("wtf is {ident}");
                                    };
                                    let offset = Statement::from_kind(StatementKind::Constant {
                                        typ: Rc::new(Type::u32()),
                                        value: rudder::ConstantValue::UnsignedInteger(reg.1),
                                    });
                                    let statement =
                                        Statement::from_kind(StatementKind::WriteRegister {
                                            offset: offset.clone(),
                                            value: call.clone(),
                                        });
                                    vec![offset, statement]
                                }
                            }
                        }
                        boom::Expression::Field { .. } => todo!(),
                        boom::Expression::Address(_) => todo!(),
                    }
                } else {
                    vec![]
                };

                stmts
                    .into_iter()
                    .chain(casts)
                    .chain([call])
                    .chain(copy)
                    .collect()
            }
            boom::Statement::Label(_)
            | boom::Statement::Goto(_)
            | boom::Statement::Jump { .. }
            | boom::Statement::End(_)
            | boom::Statement::Undefined
            | boom::Statement::If { .. } => {
                unreachable!("no control flow should exist at this point in compilation!")
            }
            boom::Statement::Exit(_) | boom::Statement::Comment(_) => vec![],
        }
    }
}

fn generate_cast(
    source: Statement,
    _source_type: Rc<Type>,
    destination_type: Rc<Type>,
) -> Statement {
    // todo: do correct kind of cast for different source/dest pairs
    Statement::from_kind(StatementKind::Cast {
        kind: CastOperationKind::Reinterpret,
        typ: destination_type,
        value: source,
    })
}

use {
    crate::{
        boom::{self, bits_to_int},
        rudder::internal_fns::{self, REPLICATE_BITS_BOREALIS_INTERNAL},
    },
    common::{
        arena::{Arena, Ref},
        id::Id,
        intern::InternedString,
        rudder::{
            block::Block,
            constant_value::ConstantValue,
            function::{Function, Symbol},
            statement::{
                build, cast, BinaryOperationKind, CastOperationKind, ShiftOperationKind, Statement,
                TernaryOperationKind, UnaryOperationKind,
            },
            types::{PrimitiveType, PrimitiveTypeClass, Type},
            Model, RegisterDescriptor,
        },
        width_helpers::signed_smallest_width_of_value,
        HashMap,
    },
    log::trace,
    num_rational::Ratio,
    num_traits::cast::FromPrimitive,
    rayon::iter::{IntoParallelIterator, ParallelIterator},
    regex::Regex,
    sailrs::shared::Shared,
    std::cmp::Ordering,
};

pub fn from_boom(ast: &boom::Ast) -> Model {
    let mut build_ctx = BuildContext::default();

    // DEFINITION ORDER DEPENDANT!!!
    ast.definitions.iter().for_each(|def| match def {
        boom::Definition::Struct { name, fields } => build_ctx.add_struct(*name, fields),
        // todo contains KV pairs, "mangled" and "tuplestruct" as keys and type names as values
        boom::Definition::Pragma { .. } => (),
    });

    ast.registers.iter().for_each(|(name, typ)| {
        let typ = build_ctx.resolve_type(typ.clone());
        build_ctx.add_register(*name, typ);
    });

    // need all functions with signatures before building
    ast.functions
        .iter()
        .for_each(|(name, definition)| build_ctx.add_function(*name, definition));

    // insert replicate bits signature

    internal_fns::insert_stub(&mut build_ctx.functions, &*REPLICATE_BITS_BOREALIS_INTERNAL);

    log::warn!("starting build functions");
    let model = build_ctx.build_functions();
    log::warn!("done build functions");

    // // insert again to overwrite empty boom generated rudder
    // model.get_functions_mut().insert(
    //     REPLICATE_BITS_BOREALIS_INTERNAL.name(),
    //     REPLICATE_BITS_BOREALIS_INTERNAL.clone(),
    // );

    model
}

#[derive(Default)]
struct BuildContext {
    /// Name of struct maps to the rudder type and a map of field names to field
    /// indices
    structs: HashMap<InternedString, (Type, HashMap<InternedString, usize>)>,

    /// Name of enum maps to the rudder type and a map of enum variants to the
    /// integer discriminant of that variant
    enums: HashMap<InternedString, (Type, HashMap<InternedString, u32>)>,

    /// Register name to type and offset mapping
    registers: HashMap<InternedString, RegisterDescriptor>,
    next_register_offset: usize,

    /// Functions
    functions: HashMap<InternedString, (Function, boom::FunctionDefinition)>,
}

impl BuildContext {
    fn add_register(&mut self, name: InternedString, typ: Type) {
        self.registers.insert(
            name,
            RegisterDescriptor {
                typ: typ.clone(),
                offset: self.next_register_offset,
            },
        );

        log::debug!("adding register {name} @ {:x}", self.next_register_offset);

        // 8 byte aligned
        self.next_register_offset += typ.width_bytes().next_multiple_of(8)
    }

    fn add_struct(&mut self, name: InternedString, fields: &[boom::NamedType]) {
        let typ = Type::Struct(
            fields
                .iter()
                .map(|boom::NamedType { name, typ }| (*name, self.resolve_type(typ.clone())))
                .collect(),
        );

        let fields = fields
            .iter()
            .enumerate()
            .map(|(idx, boom::NamedType { name, .. })| (*name, idx))
            .collect();

        if self.structs.insert(name, (typ, fields)).is_some() {
            panic!("struct with name {name} already added");
        }
    }

    fn add_function(&mut self, name: InternedString, definition: &boom::FunctionDefinition) {
        self.functions.insert(
            name,
            (
                Function::new(
                    name,
                    self.resolve_type(definition.signature.return_type.clone()),
                    definition
                        .signature
                        .parameters
                        .get()
                        .iter()
                        .map(|boom::Parameter { typ, name }| {
                            Symbol::new(*name, self.resolve_type(typ.clone()))
                        })
                        .collect(),
                ),
                definition.clone(),
            ),
        );
    }

    fn build_functions(self) -> Model {
        let fns = self
            .functions
            .clone()
            .into_par_iter()
            .map(|(name, (rudder_fn, boom_fn))| {
                log::debug!("building function {name:?}");
                (
                    name,
                    FunctionBuildContext::new(&self, rudder_fn).build_fn(boom_fn.clone()),
                )
            })
            .collect();

        Model::new(
            fns,
            // register names kept for debugging
            self.registers,
        )
    }

    fn resolve_type(&self, typ: Shared<boom::Type>) -> Type {
        match &*typ.get() {
            boom::Type::Unit => Type::unit(),
            boom::Type::String => Type::String,
            // value
            boom::Type::Bool | boom::Type::Bit => Type::u1(),
            boom::Type::Float => Type::f64(),
            boom::Type::Real => Type::Rational,
            boom::Type::Union { width } => Type::Union { width: *width },
            boom::Type::Struct { name, .. } => self.structs.get(name).unwrap().0.clone(),
            boom::Type::List { .. } => todo!(),
            boom::Type::Vector { element_type } => {
                let element_type = (self.resolve_type(element_type.clone())).clone();
                // todo: Brian Campbell said the Sail C backend had functionality to staticize
                // all bitvector lengths
                element_type.vectorize(0)
            }
            boom::Type::FixedVector {
                length,
                element_type,
            } => {
                let element_type = (self.resolve_type(element_type.clone())).clone();

                element_type.vectorize(usize::try_from(*length).unwrap())
            }
            boom::Type::Reference(inner) => {
                // todo: this is broken:(
                self.resolve_type(inner.clone())
            }
            boom::Type::Integer { size } => Type::new_primitive(
                PrimitiveTypeClass::SignedInteger,
                match size {
                    boom::Size::Static(size) => *size,
                    boom::Size::Unknown => 64,
                },
            ),
            boom::Type::Bits { size } => match size {
                boom::Size::Static(size) => {
                    Type::new_primitive(PrimitiveTypeClass::UnsignedInteger, *size)
                }
                boom::Size::Unknown => Type::Bits,
            },
            boom::Type::Constant(c) => {
                // todo: this should be a panic, but because structs/unions can have constant
                // type fields we do the following
                Type::new_primitive(
                    PrimitiveTypeClass::SignedInteger,
                    signed_smallest_width_of_value(*c).into(),
                )
            }
            boom::Type::Tuple(ts) => {
                Type::Tuple(ts.iter().cloned().map(|t| self.resolve_type(t)).collect())
            }
        }
    }
}

struct FunctionBuildContext<'ctx> {
    build_context: &'ctx BuildContext,
    rudder_fn: Function,
    blocks: HashMap<Id, Ref<Block>>,
}

impl<'ctx> FunctionBuildContext<'ctx> {
    pub fn new(build_context: &'ctx BuildContext, rudder_fn: Function) -> Self {
        Self {
            build_context,
            rudder_fn,
            blocks: HashMap::default(),
        }
    }

    pub fn build_fn(mut self, boom_fn: boom::FunctionDefinition) -> Function {
        trace!(
            "converting function {:?} from boom to rudder",
            boom_fn.signature.name
        );
        let entry = self.resolve_block(boom_fn.entry_block);
        self.rudder_fn.set_entry_block(entry);

        self.rudder_fn
    }

    pub fn resolve_block(
        &mut self,
        boom_block: boom::control_flow::ControlFlowBlock,
    ) -> Ref<Block> {
        trace!("resolving: {:x}", boom_block.id());

        if let Some(block) = self.blocks.get(&boom_block.id()) {
            trace!("already resolved: {:x}", boom_block.id());
            block.clone()
        } else {
            trace!("building: {:x}", boom_block.id());
            BlockBuildContext::new(self).build_block(boom_block)
        }
    }
}

struct BlockBuildContext<'ctx, 'fn_ctx> {
    function_build_context: &'fn_ctx mut FunctionBuildContext<'ctx>,
    block: Ref<Block>,
}

impl<'ctx: 'fn_ctx, 'fn_ctx> BlockBuildContext<'ctx, 'fn_ctx> {
    pub fn new(function_build_context: &'fn_ctx mut FunctionBuildContext<'ctx>) -> Self {
        let block = function_build_context.rudder_fn.new_block();

        Self {
            function_build_context,
            block,
        }
    }

    fn ctx(&mut self) -> &BuildContext {
        self.function_build_context.build_context
    }

    fn fn_ctx(&self) -> &FunctionBuildContext<'ctx> {
        self.function_build_context
    }

    fn fn_ctx_mut(&mut self) -> &mut FunctionBuildContext<'ctx> {
        self.function_build_context
    }

    fn block_arena_mut(&mut self) -> &mut Arena<Block> {
        self.fn_ctx_mut().rudder_fn.arena_mut()
    }

    fn build_block(mut self, boom_block: boom::control_flow::ControlFlowBlock) -> Ref<Block> {
        // pre-insert empty rudder block to avoid infinite recursion with cyclic blocks
        {
            let rudder_block = self.block.clone();
            self.fn_ctx_mut()
                .blocks
                .insert(boom_block.id(), rudder_block);
        }

        // convert statements
        boom_block
            .statements()
            .iter()
            .for_each(|stmt| self.build_statement(stmt.clone()));

        // check terminator, insert final rudder statement
        let kind = match boom_block.terminator() {
            boom::control_flow::Terminator::Return(value) => Statement::Return {
                value: self.build_value(Shared::new(value)),
            },

            boom::control_flow::Terminator::Conditional {
                condition,
                target: boom_target,
                fallthrough: boom_fallthrough,
            } => {
                let condition = self.build_value(Shared::new(condition));
                let condition = cast(self.block, self.block_arena_mut(), condition, Type::u1());

                let rudder_true_target = self.fn_ctx_mut().resolve_block(boom_target);
                let rudder_false_target = self.fn_ctx_mut().resolve_block(boom_fallthrough);

                Statement::Branch {
                    condition,
                    true_target: rudder_true_target,
                    false_target: rudder_false_target,
                }
            }
            boom::control_flow::Terminator::Unconditional {
                target: boom_target,
            } => {
                let rudder_target = self.fn_ctx_mut().resolve_block(boom_target);
                Statement::Jump {
                    target: rudder_target,
                }
            }
            boom::control_flow::Terminator::Panic(value) => {
                Statement::Panic(self.build_value(Shared::new(value.clone())))
            }
        };

        build(self.block, self.block_arena_mut(), kind);

        self.block
    }

    fn build_statement(&mut self, statement: Shared<boom::Statement>) {
        match &*statement.get() {
            boom::Statement::VariableDeclaration { name, typ } => {
                let typ = self.ctx().resolve_type(typ.clone());
                self.fn_ctx_mut()
                    .rudder_fn
                    .add_local_variable(Symbol::new(*name, typ));
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
                panic!("no control flow should exist at this point in compilation!\n{statement:?}")
            }
            boom::Statement::Exit(_) | boom::Statement::Comment(_) => (),
            boom::Statement::Panic(value) => {
                let value = self.build_value(value.clone());
                build(self.block, self.block_arena_mut(), Statement::Panic(value));
            }
        }
    }

    fn build_copy(&mut self, value: Shared<boom::Value>, expression: &boom::Expression) {
        let source = self.build_value(value.clone());

        self.build_expression_write(expression, source);
    }

    fn build_function_call(
        &mut self,
        arguments: &[Shared<boom::Value>],
        name: &InternedString,
        expression: &Option<boom::Expression>,
    ) {
        let args = arguments
            .iter()
            .map(|arg| self.build_value(arg.clone()))
            .collect::<Vec<_>>();

        let fn_statement = {
            if let Some(statement) = self.build_specialized_function(*name, &args) {
                statement
            } else {
                let return_type = self
                    .ctx()
                    .functions
                    .get(name)
                    .unwrap_or_else(|| panic!("unknown function {name:?}"))
                    .0
                    .return_type();

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Call {
                        target: *name,
                        args,
                        return_type,
                    },
                )
            }
        };

        if let Some(expression) = expression {
            match expression {
                boom::Expression::Tuple(exprs) => {
                    exprs.iter().enumerate().for_each(|(index, expression)| {
                        let tuple_field = build(
                            self.block,
                            self.block_arena_mut(),
                            Statement::TupleAccess {
                                index,
                                source: fn_statement.clone(),
                            },
                        );
                        self.build_expression_write(expression, tuple_field);
                    })
                }
                _ => self.build_expression_write(expression, fn_statement),
            }
        }
    }

    /// Sail compiler builtin functions only!
    fn build_specialized_function(
        &mut self,
        name: InternedString,
        args: &[Ref<Statement>],
    ) -> Option<Ref<Statement>> {
        if Regex::new(r"^eq_any<([0-9a-zA-Z_%<>]+)>$")
            .unwrap()
            .is_match(name.as_ref())
        {
            panic!("removed");
        } else if Regex::new(r"^plain_vector_update<([0-9a-zA-Z_%<>]+)>$")
            .unwrap()
            .is_match(name.as_ref())
        {
            Some(build(
                self.block,
                self.block_arena_mut(),
                Statement::AssignElement {
                    vector: args[0].clone(),
                    value: args[2].clone(),
                    index: args[1].clone(),
                },
            ))
        } else if Regex::new(r"^plain_vector_access<([0-9a-zA-Z_%<>]+)>$")
            .unwrap()
            .is_match(name.as_ref())
        {
            panic!("removed");
        } else {
            match name.as_ref() {
                "%i64->%i" => {
                    // lots of %i64->%i(Int(BigInt(-1))) so disabled this check
                    // assert_eq!(Type::s64(), *args[0].typ());
                    Some(args[0].clone())
                }

                "%i->%i64" => {
                    let arena = self.statement_arena();
                    assert_eq!(args[0].get(arena).typ(arena), Type::s64());

                    Some(args[0].clone())
                }

                "%string->%real" => {
                    let Statement::Constant { value, .. } = args[0].get(self.statement_arena()) else {
                        panic!();
                    };

                    let ConstantValue::String(str) = value else {
                        panic!();
                    };

                    let r = Ratio::<i128>::from_f64(str.as_ref().parse().unwrap()).unwrap();

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::Rational),
                            value: ConstantValue::Rational(r),
                        },
                    ))
                }

                "make_the_value" | "size_itself_int" => Some(args[0].clone()),
                // %bv, %i, %i -> %bv
                "subrange_bits" => {
                    // end - start + 1
                    let one = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::s64()),
                            value: ConstantValue::SignedInteger(1),
                        },
                    );

                    let typ = {
                        let arena = self.statement_arena();
                        args[1].get(arena).typ(arena)
                    };
                    let one = cast(self.block, self.block_arena_mut(), one, typ);
                    let diff = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Sub,
                            lhs: args[1].clone(),
                            rhs: args[2].clone(),
                        },
                    );
                    let len = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Add,
                            lhs: diff.clone(),
                            rhs: one.clone(),
                        },
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitExtract {
                            value: args[0].clone(),
                            start: args[2].clone(),
                            length: len,
                        },
                    ))
                }

                "undefined_range" => Some(args[0].clone()),

                "eq_bit" | "eq_bits" | "eq_int" | "eq_bool" | "eq_string" | "eq_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::CompareEqual,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                "neq_bits" | "neq_any<ESecurityState%>" | "neq_any<EFault%>" | "neq_bool" | "neq_int" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::CompareNotEqual,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                // val add_atom : (%i, %i) -> %i
                // val add_bits : (%bv, %bv) -> %bv
                // val add_real : (%real, %real) -> %real
                "add_atom" | "add_bits" | "add_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                // val add_bits_int : (%bv, %i) -> %bv
                "add_bits_int" => {
                    let rhs = cast(self.block, self.block_arena_mut(), args[1].clone(), Type::Bits);
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Add,
                            lhs: args[0].clone(),
                            rhs,
                        },
                    ))
                }

                // val sub_bits_int : (%bv, %i) -> %bv
                "sub_bits_int" => {
                    let rhs = cast(self.block, self.block_arena_mut(), args[1].clone(), Type::Bits);
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Sub,
                            lhs: args[0].clone(),
                            rhs,
                        },
                    ))
                }

                "sub_bits" | "sub_atom" | "sub_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Sub,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                "mult_atom" | "mult_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Multiply,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                "tdiv_int" | "ediv_int" | "ediv_nat" | "div_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Divide,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                "emod_nat" | "_builtin_mod_nat" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Modulo,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                "negate_atom" | "neg_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::UnaryOperation {
                        kind: UnaryOperationKind::Negate,
                        value: args[0].clone(),
                    },
                )),
                "abs_int_atom" | "abs_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::UnaryOperation {
                        kind: UnaryOperationKind::Absolute,
                        value: args[0].clone(),
                    },
                )),
                "min_int" => {
                    let true_value = args[0].clone();
                    let false_value = args[1].clone();

                    let condition = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::CompareLessThan,
                            lhs: true_value.clone(),
                            rhs: false_value.clone(),
                        },
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Select {
                            condition,
                            true_value,
                            false_value,
                        },
                    ))
                }

                "max_int" => {
                    let true_value = args[0].clone();
                    let false_value = args[1].clone();

                    let condition = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::CompareGreaterThan,
                            lhs: true_value.clone(),
                            rhs: false_value.clone(),
                        },
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Select {
                            condition,
                            true_value,
                            false_value,
                        },
                    ))
                }

                // val ceil : (%real) -> %i
                "ceil" => {
                    let ceil = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::UnaryOperation {
                            kind: UnaryOperationKind::Ceil,
                            value: args[0].clone(),
                        },
                    );
                    Some(ceil)
                }

                // val floor : (%real) -> %i
                "floor" => {
                    let floor = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::UnaryOperation {
                            kind: UnaryOperationKind::Floor,
                            value: args[0].clone(),
                        },
                    );
                    Some(floor)
                }

                // val to_real : (%i) -> %real
                "to_real" => Some(cast(
                    self.block,
                    self.block_arena_mut(),
                    args[0].clone(),
                    Type::Rational,
                )),

                // val pow2 : (%i) -> %i
                // val _builtin_pow2 : (%i) -> %i
                "pow2" | "_builtin_pow2" => {
                    // WRONG!! pow2(n) is 2^n not n^2
                    // Some(build(   self.block, self.block_arena_mut(),Statement::UnaryOperation {
                    //     kind: UnaryOperationKind::Power2,
                    //     value: args[0].clone(),
                    // }))

                    // hopefully correct
                    // 1 << args[0]
                    let const_1 = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: Type::s64(),
                            value: ConstantValue::SignedInteger(1),
                        },
                    );
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::ShiftOperation {
                            kind: ShiftOperationKind::LogicalShiftLeft,
                            value: const_1,
                            amount: args[0].clone(),
                        },
                    ))
                }

                // val pow_real : (%real, %i) -> %real
                "pow_real" => {
                    // cast args[1] to i32, todo: move this to codegen cause it's a rust thing
                    let i = cast(self.block, self.block_arena_mut(), args[1].clone(), Type::s32());

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::PowI,
                            lhs: args[0].clone(),
                            rhs: i,
                        },
                    ))
                }

                "sqrt" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::UnaryOperation {
                        kind: UnaryOperationKind::SquareRoot,
                        value: args[0].clone(),
                    },
                )),

                "lt_int" | "lt_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::CompareLessThan,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),
                "lteq_int" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::CompareLessThanOrEqual,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),
                "gt_int" | "gt_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::CompareGreaterThan,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),
                "gteq_int" | "gteq_real" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::CompareGreaterThanOrEqual,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),
                 "not_bool" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::UnaryOperation {
                        kind: UnaryOperationKind::Not,
                        value: args[0].clone(),
                    },
                )),
                "not_vec" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::UnaryOperation {
                        kind: UnaryOperationKind::Complement,
                        value: args[0].clone(),
                    },
                )),
                "and_vec" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::And,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),
                "xor_vec" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Xor,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),
                "or_vec" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Or,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    },
                )),

                "sail_shiftright" | "_shr_int" | "_shr32" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftRight,
                        value: args[0].clone(),
                        amount: args[1].clone(),
                    },
                )),
                "sail_arith_shiftright" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind: ShiftOperationKind::ArithmeticShiftRight,
                        value: args[0].clone(),
                        amount: args[1].clone(),
                    },
                )),
                "sail_shiftleft" | "_shl_int" | "_shl8" | "_shl32" | "_shl1" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftLeft,
                        value: args[0].clone(),
                        amount: args[1].clone(),
                    },
                )),

                "slice" => {
                    // uint64 n, uint64 start, uint64 len
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitExtract {
                            value: args[0].clone(),
                            start: args[1].clone(),
                            length: args[2].clone(),
                        },
                    ))
                }

                "bitvector_access" => {
                    let length = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u64()),
                            value: ConstantValue::UnsignedInteger(1),
                        },
                    );
                    let bitex = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitExtract {
                            value: args[0].clone(),
                            start: args[1].clone(),
                            length,
                        },
                    );

                    Some(cast(self.block, self.block_arena_mut(), bitex, Type::u1()))
                }

                // val undefined_bitvector : (%i) -> %bv
                "undefined_bitvector" => {
                    let zero = build( self.block,
                        self.block_arena_mut(),Statement::Constant { typ: Type::u64(), value: ConstantValue::UnsignedInteger(0) });

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::CreateBits { value:zero , length:args[0].clone() },
                    ))
                }

                "bitvector_length" => {
                    let arena = self.statement_arena();
                    assert!(matches!(args[0].get(arena).typ(arena), Type::Bits));

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::SizeOf { value: args[0].clone() },
                    ))
                }

                "update_fbits" => {
                    //     if ((bit & 1) == 1) {
                    //         return op | (bit << n);
                    //    } else {
                    //         return op & ~(bit << n);
                    //    }
                    let op = cast(self.block, self.block_arena_mut(), args[0].clone(), Type::u64());
                    let n = args[1].clone();
                    let bit = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Cast {
                            kind: CastOperationKind::ZeroExtend,
                            typ: (Type::u64()),
                            value: args[2].clone(),
                        },
                    );

                    // 1
                    let one = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u64()),
                            value: ConstantValue::UnsignedInteger(1),
                        },
                    );

                    // (bit & 1)
                    let and = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::And,
                            lhs: bit.clone(),
                            rhs: one.clone(),
                        },
                    );

                    //  (bit & 1) == 1
                    let condition = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::CompareEqual,
                            lhs: and,
                            rhs: one,
                        },
                    );

                    // bit << n
                    let shift = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::ShiftOperation {
                            kind: ShiftOperationKind::LogicalShiftLeft,
                            value: bit.clone(),
                            amount: n,
                        },
                    );

                    // op | (bit << n)
                    let true_value = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Or,
                            lhs: op.clone(),
                            rhs: shift.clone(),
                        },
                    );

                    // ~(bit << n)
                    let inverse = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::UnaryOperation {
                            kind: UnaryOperationKind::Complement,
                            value: shift,
                        },
                    );

                    // op & ~(bit << n)
                    let false_value = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::And,
                            lhs: op,
                            rhs: inverse,
                        },
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Select {
                            condition,
                            true_value,
                            false_value,
                        },
                    ))
                }

                // %bv -> %i
                "UInt0" | "unsigned" | "_builtin_unsigned" => {
                    // just copy bits

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Cast {
                            kind: CastOperationKind::Reinterpret,
                            typ: Type::s64(),
                            value: args[0].clone(),
                        },
                    ))
                }
                // %bv -> %i
                "SInt0" => {
                    //                     void sail_signed(sail_int *rop, const lbits op)
                    // {
                    //   if (op.len == 0) {
                    //     mpz_set_ui(*rop, 0);
                    //   } else {
                    //     mp_bitcnt_t sign_bit = op.len - 1;
                    //     mpz_set(*rop, *op.bits);
                    //     if (mpz_tstbit(*op.bits, sign_bit) != 0) {
                    //       /* If sign bit is unset then we are done,
                    //          otherwise clear sign_bit and subtract 2**sign_bit */
                    //       mpz_set_ui(sail_lib_tmp1, 1);
                    //       mpz_mul_2exp(sail_lib_tmp1, sail_lib_tmp1, sign_bit); /* 2**sign_bit */
                    //       mpz_combit(*rop, sign_bit); /* clear sign_bit */
                    //       mpz_sub(*rop, *rop, sail_lib_tmp1);
                    //     }
                    //   }
                    // }
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Cast {
                            kind: CastOperationKind::SignExtend,
                            typ: Type::s64(),
                            value: args[0].clone(),
                        },
                    ))
                }

                // val ZeroExtend0 : (%bv, %i) -> %bv
                // val sail_zero_extend : (%bv, %i) -> %bv
                "ZeroExtend0" | "sail_zero_extend" => {
                    let length = args[1].get(self.statement_arena());
                    if let Statement::Constant { value, .. } = length {
                        let width = match value {
                            ConstantValue::UnsignedInteger(u) => usize::try_from(*u).unwrap(),
                            ConstantValue::SignedInteger(i) => usize::try_from(*i).unwrap(),
                            _ => panic!(),
                        };
                        Some(build(
                            self.block,
                            self.block_arena_mut(),
                            Statement::Cast {
                                kind: CastOperationKind::ZeroExtend,
                                typ: (Type::new_primitive(PrimitiveTypeClass::UnsignedInteger, width)),
                                value: args[0].clone(),
                            },
                        ))
                    } else {
                        Some(build(
                            self.block,
                            self.block_arena_mut(),
                            Statement::BitsCast {
                                kind: CastOperationKind::ZeroExtend,
                                typ: (Type::Bits),
                                value: args[0],
                                length: args[1],
                            },
                        ))
                    }
                }

                // val SignExtend0 : (%bv, %i) -> %bv
                // val sail_sign_extend : (%bv, %i) -> %bv
                "SignExtend0" | "sail_sign_extend" => {
                    match (args[0].get(self.statement_arena()).typ(self.statement_arena()).clone(), args[1].get(self.statement_arena()).clone()) {
                        (Type::Primitive(PrimitiveType { tc: PrimitiveTypeClass::UnsignedInteger, .. }), Statement::Constant { value: ConstantValue::SignedInteger(length),.. }) =>

                           Some(build(self.block, self.block_arena_mut(), Statement::Cast { kind: CastOperationKind::SignExtend, typ: Type::Primitive(PrimitiveType { tc: PrimitiveTypeClass::UnsignedInteger, element_width_in_bits:usize::try_from(length).unwrap() }), value: args[0] }))
                        ,
                        (Type::Bits,_) =>  Some(build(
                            self.block,
                            self.block_arena_mut(),
                            Statement::BitsCast {
                                kind: CastOperationKind::SignExtend,
                                typ: (Type::Bits),
                                value: args[0].clone(),
                                length: args[1].clone(),
                            },
                        )),
                        (_,_) => todo!("sail extend {:?} {:?}",args[0], args[1]),
                    }
                   },

                // val truncate : (%bv, %i) -> %bv
                "truncate" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BitsCast {
                        kind: CastOperationKind::Truncate,
                        typ: (Type::Bits),
                        value: args[0].clone(),
                        length: args[1].clone(),
                    },
                )),

                "sail_zeros" => {
                    let length = args[0].clone();

                    let const_0 = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u8()),
                            value: ConstantValue::UnsignedInteger(0),
                        },
                    );

                    let value = cast(self.block, self.block_arena_mut(), const_0, Type::Bits);

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitsCast {
                            kind: CastOperationKind::ZeroExtend,
                            typ: (Type::Bits),
                            value,
                            length,
                        },
                    ))
                }

                "sail_assert" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Assert {
                        condition: args[0].clone(),
                    },
                )),


                // val bitvector_update : (%bv, %i, %bit) -> %bv
                "bitvector_update" => {
                    let target = cast(self.block, self.block_arena_mut(), args[0].clone(), Type::Bits);
                    let i = args[1].clone();
                    let bit = cast(self.block, self.block_arena_mut(), args[2].clone(), Type::Bits);

                    let const_1 = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u64()),
                            value: ConstantValue::UnsignedInteger(1),
                        },
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitInsert {
                            target,
                            source: bit,
                            start: i,
                            length: const_1,
                        },
                    ))
                }

                // val append_64 : (%bv, %bv64) -> %bv
                "append_64" => {
                    Some(self.generate_concat(args[0].clone(),args[1].clone()))
                }

                "bitvector_concat" => Some(self.generate_concat(args[0].clone(), args[1].clone())),

                // val set_slice_int : (%i, %i, %i, %bv) -> %i
                "set_slice_int" => {
                    // const sail_int len, const sail_int n, const sail_int start, const lbits slice
                    let len = args[0].clone();
                    let n = args[1].clone();
                    let start = args[2].clone();

                    // cast the slice from a %bv to a i128 for the i128 version of bit-insert
                    let slice = args[3].clone();

                    // destination[start..] = source[0..source.len()]
                    // todo: check correctness and write some unit tests for this

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitInsert {
                            target: n,
                            source: slice,
                            start,
                            length: len,
                        },
                    ))
                }

                //val get_slice_int : (%i, %i, %i) -> %bv
                "get_slice_int" => {
                    let extract = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitExtract {
                            value: args[1].clone(),
                            start: args[2].clone(),
                            length: args[0].clone(),
                        },
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::CreateBits { value:extract, length:args[0].clone() },
                    ))
                }

                // val set_slice_bits : (%i, %i, %bv, %i, %bv) -> %bv
                "set_slice_bits" => {
                    // len, slen, x, pos, y
                    let _len = args[0].clone();
                    let slen = args[1].clone();
                    let destination = args[2].clone();
                    let start = args[3].clone();
                    let source = args[4].clone();

                    // destination[start..] = source[0..source.len()]
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitInsert {
                            target: destination,
                            source,
                            start,
                            length: slen,
                        },
                    ))
                }

                "update_subrange_bits" => {
                    let destination = args[0].clone();
                    let end = args[1].clone();
                    let start = args[2].clone();
                    let source = args[3].clone();

                    let sum = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Sub,
                            lhs: end,
                            rhs: start.clone(),
                        },
                    );

                    let const_1 =build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u64()),
                            value: ConstantValue::UnsignedInteger(1),
                        },
                    );


                    let source_length = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Add,
                            lhs: sum,
                            rhs: const_1,
                        },
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitInsert {
                            target: destination,
                            source,
                            start,
                            length: source_length,
                        },
                    ))
                }

                "replicate_bits" => {
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Call {
                            target: REPLICATE_BITS_BOREALIS_INTERNAL.name(),
                            args: vec![args[0].clone(), args[1].clone()],
                            return_type: REPLICATE_BITS_BOREALIS_INTERNAL.return_type(),
                        },
                    ))
                }

                /* ### NON-BUILTIN FUNCTIONS BELOW THIS POINT ### */
                "AddWithCarry" => {
                    let x = args[0].clone();
                    let y = args[1].clone();
                    let carry_in = args[2].clone();

                    let sum = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::TernaryOperation { kind: TernaryOperationKind::AddWithCarry, a: x, b: y, c: carry_in},
                    );

                    // nzcv bv4
                    let flags = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::GetFlags {operation: sum.clone()},
                    );

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::CreateTuple(vec![sum, flags]),
                    ))
                }

                /* To maintain correctness, borealis must only specialize on actual Sail compiler builtins, specializing other functions means restricting compatibiliy on a specific model, however memory access simply must be overwritten */
                "read_mem_exclusive#<RMem_read_request<Uarm_acc_type<>,b,O<RTranslationInfo>>>"
                | "read_mem_ifetch#<RMem_read_request<Uarm_acc_type<>,b,O<RTranslationInfo>>>"
                | "read_mem#<RMem_read_request<Uarm_acc_type<>,b,O<RTranslationInfo>>>" => {
                    let _request = args[0].clone();
                    let _addrsize = args[1].clone();
                    let phys_addr = args[2].clone();
                    let n = args[3].clone();

                    let size_bytes = cast(self.block, self.block_arena_mut(), n, Type::u64());

                    let const_8 = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u64()),
                            value: ConstantValue::UnsignedInteger(8),
                        },
                    );
                    let size_bits = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Multiply,
                            lhs: size_bytes,
                            rhs: const_8,
                        },
                    );

                    let offset = cast(self.block, self.block_arena_mut(), phys_addr, Type::u64());

                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::ReadMemory {
                            offset,
                            size: size_bits,
                        },
                    ))
                }

                "write_mem_exclusive#<RMem_write_request<Uarm_acc_type<>,b,O<RTranslationInfo>>>"
                | "write_mem#<RMem_write_request<Uarm_acc_type<>,b,O<RTranslationInfo>>>" => {
                    let _request = args[0].clone();
                    let _addrsize = args[1].clone();
                    let phys_addr = args[2].clone();
                    let n = args[3].clone();
                    let data = args[4].clone();

                    let size_bytes = cast(self.block, self.block_arena_mut(), n, Type::u64());

                    let const_8 = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u64()),
                            value: ConstantValue::UnsignedInteger(8),
                        },
                    );
                    let size_bits = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BinaryOperation {
                            kind: BinaryOperationKind::Multiply,
                            lhs: size_bytes,
                            rhs: const_8,
                        },
                    );

                    let value = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::BitsCast {
                            kind: CastOperationKind::Truncate,
                            typ: Type::Bits,
                            value: data,
                            length: size_bits,
                        },
                    );
                    let offset = cast(self.block, self.block_arena_mut(), phys_addr, Type::u64());

                    build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::WriteMemory { offset, value },
                    );

                    // return value also appears to be always ignored
                    Some(build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u1()),
                            value: ConstantValue::UnsignedInteger(0),
                        },
                    ))
                }

                // ignore
                "append_str" | "__monomorphize" | "concat_str" => Some(args[0].clone()),

                // result of sail_mem_read always appears to ignore the value returned by `read_tag#` (underscore in Ok((value, _))):
                // match sail_mem_read(read_request(accdesc, translation_info, size, desc.vaddress, desc.paddress.address)) {
                //     Ok((value, _)) => (CreatePhysMemRetStatus(Fault_None), value),
                //     Err(statuscode) => (CreatePhysMemRetStatus(statuscode), sail_zeros(8 * size))
                //   }
                "read_tag#" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::u1()),
                        value: ConstantValue::UnsignedInteger(1),
                    },
                )),
                "write_tag#" => {
                    let msg = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::String),
                            value: ConstantValue::String("write_tag panic".into()),
                        },
                    );
                    Some(build(self.block, self.block_arena_mut(), Statement::Panic(msg)))
                }

                "DecStr" | "bits_str" | "HexStr" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::String),
                        value: ConstantValue::String("fix me in build_specialized_function".into()),
                    },
                )),

                // todo: remove me!
                "HaveBRBExt" | "HaveStatisticalProfiling" | "HaveGCS" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::u1()),
                        value: ConstantValue::UnsignedInteger(0),
                    },
                )),

                "__GetVerbosity" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::u64()),
                        value: ConstantValue::UnsignedInteger(0),
                    },
                )),

                "get_cycle_count" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: Type::s64(),
                        value: ConstantValue::SignedInteger(0),
                    },
                )),

                // requires u256 internally :(
                "SHA256hash" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::new_primitive(PrimitiveTypeClass::UnsignedInteger, 256)),
                        value: ConstantValue::UnsignedInteger(0),
                    },
                )),

                // val putchar : (%i) -> %unit
                "putchar" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Panic(args[0].clone()),
                )),

                "AArch64_DC"
                | "execute_aarch64_instrs_system_barriers_dmb"
                | "execute_aarch64_instrs_system_barriers_dsb"
                | "execute_aarch64_instrs_system_barriers_isb"
                | "sail_return_exception"
                | "sail_branch_announce"
                | "sail_tlbi"
                | "prerr_bits"
                | "prerr_int"
                | "sail_cache_op"
                | "sail_barrier"
                | "__WakeupRequest"
                | "print"
                | "print_endline"
                | "check_cycle_count"
                | "sail_take_exception" => Some(build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::unit()),
                        value: ConstantValue::Unit,
                    },
                )),
                _ => None,
            }
        }
    }

    // fn build_union_constructor(
    //     &mut self,
    //     name: InternedString,
    //     args: &[Statement],
    // ) -> Option<Statement> {
    //     self.ctx()
    //         .unions
    //         .values()
    //         .find(|(_, variants)| variants.contains_key(&name))
    //         .map(|(typ, _)| typ)
    //         .cloned()
    //         .map(|typ| {
    //             build(   self.block,
    // self.block_arena_mut(),Statement::CreateEnum {                 typ,
    //                 variant: name,
    //                 value: args[0].clone(),
    //             })
    //         })
    // }

    /// Generates rudder for a writing a statement to a boom::Expression
    fn build_expression_write(&mut self, target: &boom::Expression, source: Ref<Statement>) {
        let idents = expression_field_collapse(target);
        let (root, fields) = idents
            .split_first()
            .expect("expression should always at least contain the root");

        if !fields.is_empty() {
            panic!("{root} {fields:?}");
        }

        match self.fn_ctx_mut().rudder_fn.get_local_variable(*root) {
            Some(symbol) => {
                let (_, outer_type) = fields_to_indices(&self.ctx().structs, symbol.typ(), fields);

                let value = cast(self.block, self.block_arena_mut(), source, outer_type);

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::WriteVariable { symbol, value },
                );
            }
            None => {
                //register lookup
                let Some(RegisterDescriptor {
                    typ: register_type,
                    offset: register_offset,
                    ..
                }) = self.ctx().registers.get(root).cloned()
                else {
                    panic!("wtf is {root} in {}", self.fn_ctx().rudder_fn.name());
                };

                let (field_offsets, outer_type) =
                    fields_to_offsets(&self.ctx().structs, register_type, fields);

                // offset + offset of each field
                let offset = register_offset + field_offsets.iter().sum::<usize>();

                // cast to outermost type
                let cast = cast(self.block, self.block_arena_mut(), source, outer_type);

                let offset = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::u32()),
                        value: ConstantValue::UnsignedInteger(u64::try_from(offset).unwrap()),
                    },
                );

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::WriteRegister {
                        offset,
                        value: cast,
                    },
                );
            }
        }
    }

    /// Last statement returned is the value
    fn build_value(&mut self, boom_value: Shared<boom::Value>) -> Ref<Statement> {
        let (base, outer_field_accesses) = value_field_collapse(boom_value.clone());

        assert!(outer_field_accesses.is_empty());

        let borrow = base.get();

        match &*borrow {
            boom::Value::Identifier(ident) => {
                // local variable
                if let Some(symbol) = self.fn_ctx_mut().rudder_fn.get_local_variable(*ident) {
                    return build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::ReadVariable { symbol },
                    );
                }

                // parameter
                if let Some(symbol) = self.fn_ctx_mut().rudder_fn.get_parameter(*ident) {
                    return build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::ReadVariable { symbol },
                    );
                }

                // register
                if let Some(RegisterDescriptor {
                    typ,
                    offset: register_offset,
                    ..
                }) = self.ctx().registers.get(ident).cloned()
                {
                    let (offsets, outer_type) =
                        fields_to_offsets(&self.ctx().structs, typ.clone(), &outer_field_accesses);

                    let offset = register_offset + offsets.iter().sum::<usize>();

                    let offset = build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::Constant {
                            typ: (Type::u32()),
                            value: ConstantValue::UnsignedInteger(u64::try_from(offset).unwrap()),
                        },
                    );

                    return build(
                        self.block,
                        self.block_arena_mut(),
                        Statement::ReadRegister {
                            typ: outer_type,
                            offset,
                        },
                    );
                }

                // enum
                if let Some(_) = self
                    .ctx()
                    .enums
                    .iter()
                    .find_map(|(_, (_, variants))| variants.get(ident))
                    .cloned()
                {
                    panic!("these should be members now?");
                }

                panic!("unknown ident: {:?}\n{:?}", ident, boom_value);
            }

            boom::Value::Literal(literal) => {
                assert!(outer_field_accesses.is_empty());
                self.build_literal(&literal.get())
            }
            boom::Value::Operation(op) => {
                assert!(outer_field_accesses.is_empty());
                self.build_operation(op)
            }
            boom::Value::Tuple(values) => {
                let values = values.iter().map(|v| self.build_value(v.clone())).collect();
                return build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::CreateTuple(values),
                );
            }
            boom::Value::Struct { name, fields } => {
                panic!("got struct {name} {fields:?} but structs should have been removed in boom")
            }

            boom::Value::Field { .. } => panic!("fields should have already been flattened"),

            // return false if `value`` is of the variant `identifier`, else true
            boom::Value::CtorKind { .. } => {
                // assert!(outer_field_accesses.is_empty());

                // let value = self.build_value(value.clone());

                // // get the rudder type
                // let typ = self
                //     .ctx()
                //     .unions
                //     .values()
                //     .find(|(_, variants)| variants.contains_key(identifier))
                //     .map(|(typ, _)| typ)
                //     .cloned()
                //     .unwrap();

                // assert_eq!(value.typ(), typ);

                // // todo: investigate this further
                // let matches = build(   self.block,
                // self.block_arena_mut(),Statement::MatchesUnion {
                //     value,
                //     variant: *identifier,
                // });
                // build(   self.block,   self.block_arena_mut(),Statement::UnaryOperation {
                //     kind: UnaryOperationKind::Not,
                //     value: matches,
                // })
                todo!()
            }
            boom::Value::CtorUnwrap { .. } => {
                // let value = self.build_value(value.clone());

                // // get the rudder type
                // let typ = self
                //     .ctx()
                //     .unions
                //     .values()
                //     .find(|(_, variants)| variants.contains_key(identifier))
                //     .map(|(typ, _)| typ)
                //     .cloned()
                //     .unwrap();

                // assert_eq!(value.typ(), typ);

                // let unwrap_sum = build(   self.block,
                // self.block_arena_mut(),Statement::UnwrapUnion {
                //     value,
                //     variant: *identifier,
                // });

                // unwrap_sum
                todo!()
            }
            boom::Value::VectorAccess { value, index } => {
                let vector = self.build_value(value.clone());
                let index = self.build_value(index.clone());
                return build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ReadElement { vector, index },
                );
            }
            boom::Value::VectorMutate {
                vector,
                element,
                index,
            } => {
                let vector = self.build_value(vector.clone());
                let index = self.build_value(index.clone());
                let element = self.build_value(element.clone());
                return build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::AssignElement {
                        vector,
                        value: element,
                        index,
                    },
                );
            }
        }
    }

    fn build_literal(&mut self, literal: &boom::Literal) -> Ref<Statement> {
        let kind = match literal {
            boom::Literal::Int(i) => {
                let value = i.try_into().unwrap_or_else(|_| {
                    log::error!("failed to convert {i} to i64");
                    i64::MAX
                });
                Statement::Constant {
                    typ: (Type::new_primitive(PrimitiveTypeClass::SignedInteger, 64)),
                    value: ConstantValue::SignedInteger(value),
                }
            }
            boom::Literal::Bits(bits) => Statement::Constant {
                typ: (Type::new_primitive(PrimitiveTypeClass::UnsignedInteger, bits.len())),
                value: ConstantValue::UnsignedInteger(bits_to_int(bits).try_into().unwrap()),
            },
            boom::Literal::Bit(bit) => Statement::Constant {
                typ: (Type::u1()),
                value: ConstantValue::UnsignedInteger(bit.value().try_into().unwrap()),
            },
            boom::Literal::Bool(b) => Statement::Constant {
                typ: (Type::u1()),
                value: ConstantValue::UnsignedInteger(if *b { 1 } else { 0 }),
            },
            boom::Literal::String(str) => Statement::Constant {
                typ: (Type::String),
                value: ConstantValue::String(*str),
            },
            boom::Literal::Unit => Statement::Constant {
                typ: (Type::unit()),
                value: ConstantValue::Unit,
            },
            boom::Literal::Reference(_) => todo!(),
            boom::Literal::Undefined => Statement::Undefined,
        };

        build(self.block, self.block_arena_mut(), kind)
    }

    fn build_operation(&mut self, op: &boom::Operation) -> Ref<Statement> {
        match op {
            boom::Operation::Not(value) => {
                let value = self.build_value(value.clone());
                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::UnaryOperation {
                        kind: UnaryOperationKind::Not,
                        value,
                    },
                )
            }
            boom::Operation::Complement(value) => {
                let value = self.build_value(value.clone());
                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::UnaryOperation {
                        kind: UnaryOperationKind::Complement,
                        value,
                    },
                )
            }
            boom::Operation::Cast(value, typ) => {
                let target_type = self.ctx().resolve_type(typ.clone());
                let value = self.build_value(value.clone());

                let source_type = value
                    .get(self.statement_arena())
                    .typ(self.statement_arena());

                let kind = match source_type {
                    Type::Struct(_) | Type::Vector { .. } | Type::String => {
                        panic!("cast on non-primitive type")
                    }
                    Type::Primitive(_) => {
                        match source_type.width_bits().cmp(&target_type.width_bits()) {
                            Ordering::Less => CastOperationKind::ZeroExtend,
                            Ordering::Greater => CastOperationKind::Truncate,
                            Ordering::Equal => CastOperationKind::Reinterpret,
                        }
                    }
                    Type::Bits | Type::Rational | Type::Any => {
                        todo!()
                    }
                    Type::Union { .. } => todo!(),
                    Type::Tuple(_) => todo!(),
                };

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Cast {
                        kind,
                        typ: target_type,
                        value,
                    },
                )
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

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind,
                        value,
                        amount,
                    },
                )
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

                let arena = self.statement_arena();

                if lhs.get(arena).typ(arena) != rhs.get(arena).typ(arena) {
                    // need to insert casts
                    let destination_type = if lhs.get(arena).typ(arena).width_bits()
                        > rhs.get(arena).typ(arena).width_bits()
                    {
                        lhs.get(arena).typ(arena)
                    } else {
                        rhs.get(arena).typ(arena)
                    };

                    lhs = cast(
                        self.block,
                        self.block_arena_mut(),
                        lhs.clone(),
                        destination_type.clone(),
                    );
                    rhs = cast(
                        self.block,
                        self.block_arena_mut(),
                        rhs.clone(),
                        destination_type,
                    );
                }

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: match op {
                            boom::Operation::Equal(_, _) => BinaryOperationKind::CompareEqual,
                            boom::Operation::NotEqual(_, _) => BinaryOperationKind::CompareNotEqual,
                            boom::Operation::LessThan(_, _) => BinaryOperationKind::CompareLessThan,
                            boom::Operation::LessThanOrEqual(_, _) => {
                                BinaryOperationKind::CompareLessThanOrEqual
                            }
                            boom::Operation::GreaterThan(_, _) => {
                                BinaryOperationKind::CompareGreaterThan
                            }
                            boom::Operation::GreaterThanOrEqual(_, _) => {
                                BinaryOperationKind::CompareGreaterThanOrEqual
                            }
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
                    },
                )
            }
        }
    }

    fn generate_concat(&mut self, left: Ref<Statement>, right: Ref<Statement>) -> Ref<Statement> {
        let arena = self.statement_arena();

        // todo: (zero extend original value || create new bits with runtime length)
        // then bitinsert
        match (left.get(arena).typ(arena), right.get(arena).typ(arena)) {
            (Type::Bits, Type::Bits) => {
                let l_value = cast(
                    self.block,
                    self.block_arena_mut(),
                    left.clone(),
                    Type::u128(),
                );
                let l_length = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::SizeOf { value: left },
                );

                let r_value = cast(
                    self.block,
                    self.block_arena_mut(),
                    right.clone(),
                    Type::u128(),
                );
                let r_length = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::SizeOf { value: right },
                );

                let shift = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftLeft,
                        value: l_value,
                        amount: r_length.clone(),
                    },
                );

                let value = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Or,
                        lhs: shift,
                        rhs: r_value,
                    },
                );
                let length = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: l_length,
                        rhs: r_length,
                    },
                );

                // lhs.value << rhs.len | rhs.value
                // lhs.len + rhs.len
                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::CreateBits { value, length },
                )
            }
            (
                Type::Primitive(PrimitiveType {
                    tc: PrimitiveTypeClass::UnsignedInteger,
                    element_width_in_bits: left_width,
                }),
                Type::Primitive(PrimitiveType {
                    tc: PrimitiveTypeClass::UnsignedInteger,
                    element_width_in_bits: right_width,
                }),
            ) => {
                // cast left to width left + right
                // shift left by width of right
                // OR in right

                let left_cast = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Cast {
                        kind: CastOperationKind::ZeroExtend,
                        typ: (Type::Primitive(PrimitiveType {
                            tc: PrimitiveTypeClass::UnsignedInteger,
                            element_width_in_bits: left_width + right_width,
                        })),
                        value: left,
                    },
                );

                let right_width_constant = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: (Type::u16()),
                        value: ConstantValue::UnsignedInteger(u64::try_from(right_width).unwrap()),
                    },
                );

                let left_shift = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftLeft,
                        value: left_cast,
                        amount: right_width_constant,
                    },
                );

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Or,
                        lhs: left_shift,
                        rhs: right,
                    },
                )
            }
            (
                Type::Primitive(PrimitiveType {
                    tc: PrimitiveTypeClass::UnsignedInteger,
                    element_width_in_bits: left_width,
                }),
                Type::Bits,
            ) => {
                let right_width = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::SizeOf { value: right },
                );

                let left_width = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: Type::u16(),
                        value: ConstantValue::UnsignedInteger(u64::try_from(left_width).unwrap()),
                    },
                );

                let length = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: left_width,
                        rhs: right_width,
                    },
                );

                let left_cast = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::CreateBits {
                        value: left,
                        length,
                    },
                );

                let left_shift = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftLeft,
                        value: left_cast,
                        amount: right_width,
                    },
                );
                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Or,
                        lhs: left_shift,
                        rhs: right,
                    },
                )
            }
            (
                Type::Bits,
                Type::Primitive(PrimitiveType {
                    tc: PrimitiveTypeClass::UnsignedInteger,
                    element_width_in_bits: right_width,
                }),
            ) => {
                let right_width = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::Constant {
                        typ: Type::u16(),
                        value: ConstantValue::UnsignedInteger(u64::try_from(right_width).unwrap()),
                    },
                );

                let left_shift = build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftLeft,
                        value: left,
                        amount: right_width,
                    },
                );

                build(
                    self.block,
                    self.block_arena_mut(),
                    Statement::BinaryOperation {
                        kind: BinaryOperationKind::Or,
                        lhs: left_shift,
                        rhs: right,
                    },
                )
            }
            (a, b) => panic!("todo concat for {a:?} {b:?}"),
        }
    }

    fn statement_arena(&self) -> &Arena<Statement> {
        self.block.get(self.fn_ctx().rudder_fn.arena()).arena()
    }
}

/// Function to collapse nested expression fields
///
/// Returns the root identifier followed by any and all fields
fn expression_field_collapse(expression: &boom::Expression) -> Vec<InternedString> {
    let mut result = vec![];

    let mut current_expression = expression;

    loop {
        match current_expression {
            boom::Expression::Identifier(ident) => {
                result.push(*ident);
                result.reverse();
                return result;
            }
            boom::Expression::Field { expression, field } => {
                result.push(*field);
                current_expression = expression;
            }
            boom::Expression::Address(_) => panic!("addresses not supported"),
            boom::Expression::Tuple(_) => todo!(),
        }
    }
}

/// Function to collapse nested value fields
///
/// Returns the base value and a vec of field accesses
fn value_field_collapse(value: Shared<boom::Value>) -> (Shared<boom::Value>, Vec<InternedString>) {
    let mut fields = vec![];

    let mut current_value = value;

    loop {
        // get next value and field name out of current value
        // done this way to avoid borrow issues
        let extract = match &*current_value.get() {
            boom::Value::Field { value, field_name } => Some((value.clone(), *field_name)),
            _ => None,
        };

        // if there waas one, push field and update current value
        if let Some((new_value, field_name)) = extract {
            fields.push(field_name);
            current_value = new_value;
        } else {
            // otherwise hit end so return
            fields.reverse();
            return (current_value, fields);
        }
    }
}

/// Given a type and array of field accesses, produce a corresponding array of
/// indices to each field access, and the type of the outermost access
fn fields_to_indices(
    structs: &HashMap<InternedString, (Type, HashMap<InternedString, usize>)>,
    initial_type: Type,
    fields: &[InternedString],
) -> (Vec<usize>, Type) {
    let mut current_type = initial_type;

    let mut indices = vec![];

    fields.iter().for_each(|field| {
        // get the fields of the current struct
        let (_, (struct_typ, fields)) = structs
            .iter()
            .find(|(_, (candidate, _))| current_type == *candidate)
            .expect("failed to find struct :(");

        // get index and push
        let idx = *fields.get(field).unwrap();
        indices.push(idx);

        // update current struct to point to field
        let Type::Struct(fields) = struct_typ else {
            panic!("cannot get fields of non-product")
        };
        current_type = fields[idx].1.clone();
    });

    (indices, current_type)
}

fn fields_to_offsets(
    structs: &HashMap<InternedString, (Type, HashMap<InternedString, usize>)>,
    initial_type: Type,
    fields: &[InternedString],
) -> (Vec<usize>, Type) {
    let mut current_type = initial_type;

    let mut offsets = vec![];

    fields.iter().for_each(|field| {
        // get the fields of the current struct
        let (_, (_, fields)) = structs
            .iter()
            .find(|(_, (candidate, _))| current_type == *candidate)
            .expect("failed to find struct :(");

        // get index and push
        let idx = *fields.get(field).unwrap();
        offsets.push(current_type.byte_offset(idx).unwrap());

        // update current struct to point to field
        let Type::Struct(fields) = &current_type else {
            panic!("cannot get fields of non-product")
        };
        current_type = fields[idx].1.clone();
    });

    (offsets, current_type)
}

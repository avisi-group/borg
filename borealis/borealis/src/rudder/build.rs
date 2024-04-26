use {
    crate::{
        boom::{
            self, bits_to_int, control_flow::ControlFlowBlock, FunctionSignature, NamedType,
            NamedValue,
        },
        rudder::{
            self,
            internal_fns::{
                REPLICATE_BITS_BOREALIS_INTERNAL, REPLICATE_BITS_BOREALIS_INTERNAL_NAME,
            },
            BinaryOperationKind, Block, CastOperationKind, ConstantValue, Context, Function,
            FunctionInner, FunctionKind, ShiftOperationKind, Statement, StatementBuilder,
            StatementKind, Type, UnaryOperationKind,
        },
    },
    common::{identifiable::Id, intern::InternedString, shared::Shared, HashMap},
    log::trace,
    regex::Regex,
    std::{cmp::Ordering, sync::Arc},
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

    // insert replicate bits signature
    build_ctx.functions.insert(
        *REPLICATE_BITS_BOREALIS_INTERNAL_NAME,
        (
            FunctionKind::Execute,
            rudder::Function {
                inner: Shared::new(FunctionInner {
                    name: *REPLICATE_BITS_BOREALIS_INTERNAL_NAME,
                    return_type: REPLICATE_BITS_BOREALIS_INTERNAL.return_type(),
                    parameters: REPLICATE_BITS_BOREALIS_INTERNAL
                        .inner
                        .get()
                        .parameters
                        .clone(),
                    local_variables: HashMap::default(),
                    entry_block: Block::new(),
                }),
            },
            boom::FunctionDefinition {
                signature: FunctionSignature {
                    name: *REPLICATE_BITS_BOREALIS_INTERNAL_NAME,
                    parameters: Shared::new(vec![]),
                    return_type: Shared::new(boom::Type::Unit),
                },
                entry_block: ControlFlowBlock::new(),
            },
        ),
    );

    log::warn!("starting build functions");
    build_ctx.build_functions();
    log::warn!("done build functions");

    // insert again to overwrite empty boom generated rudder
    build_ctx
        .functions
        .get_mut(&REPLICATE_BITS_BOREALIS_INTERNAL_NAME)
        .unwrap()
        .1 = REPLICATE_BITS_BOREALIS_INTERNAL.clone();

    build_ctx.finalise()
}

#[derive(Default)]
struct BuildContext {
    /// Name of struct maps to the rudder type and a map of field names to field
    /// indices
    structs: HashMap<InternedString, (Arc<rudder::Type>, HashMap<InternedString, usize>)>,
    /// Name of union maps to the rudder type and a map of field names to field
    /// indices
    unions: HashMap<InternedString, (Arc<rudder::Type>, HashMap<InternedString, usize>)>,
    /// Name of enum maps to the rudder type and a map of enum variants to the
    /// integer discriminant of that variant
    enums: HashMap<InternedString, (Arc<rudder::Type>, HashMap<InternedString, u32>)>,

    /// Register name to type and offset mapping
    registers: HashMap<InternedString, (Arc<rudder::Type>, usize)>,
    next_register_offset: usize,

    /// Functions
    functions: HashMap<InternedString, (FunctionKind, Function, boom::FunctionDefinition)>,
}

impl BuildContext {
    fn add_register(&mut self, name: InternedString, typ: Arc<Type>) {
        self.registers
            .insert(name, (typ.clone(), self.next_register_offset));

        log::debug!("adding register {name} @ {:x}", self.next_register_offset);

        // 8 byte aligned
        self.next_register_offset += typ.width_bytes().next_multiple_of(8)
    }

    fn add_struct(&mut self, name: InternedString, fields: &[boom::NamedType]) {
        let typ = Arc::new(Type::Composite(
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
        let typ = Arc::new(Type::Composite(
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
        let typ = Arc::new(Type::u32());

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
                    definition.signature.parameters.get().iter().map(
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

    fn resolve_type(&self, typ: Shared<boom::Type>) -> Arc<rudder::Type> {
        match &*typ.get() {
            boom::Type::Unit => Arc::new(rudder::Type::unit()),
            boom::Type::String => Arc::new(rudder::Type::u32()), /* same as InternedString raw */
            // value
            boom::Type::Bool | boom::Type::Bit => Arc::new(rudder::Type::u1()),
            boom::Type::Real => Arc::new(rudder::Type::f32()),
            boom::Type::Float => Arc::new(rudder::Type::f64()),
            boom::Type::Constant(_) => todo!(),
            boom::Type::Enum { name, .. } => self.enums.get(name).unwrap().0.clone(),
            boom::Type::Union { name, .. } => self.unions.get(name).unwrap().0.clone(),
            boom::Type::Struct { name, .. } => self.structs.get(name).unwrap().0.clone(),
            boom::Type::List { .. } => todo!(),
            boom::Type::Vector { element_type } => {
                let element_type = (*self.resolve_type(element_type.clone())).clone();
                // todo: Brian Campbell said the Sail C backend had functionality to staticize
                // all bitvector lengths
                Arc::new(element_type.vectorize(0))
            }
            boom::Type::FixedVector {
                length,
                element_type,
            } => {
                let element_type = (*self.resolve_type(element_type.clone())).clone();

                Arc::new(element_type.vectorize(usize::try_from(*length).unwrap()))
            }
            boom::Type::Reference(inner) => {
                // todo: this is broken:(
                self.resolve_type(inner.clone())
            }
            boom::Type::Integer { size } => match size {
                boom::Size::Static(size) => Arc::new(rudder::Type::new_primitive(
                    rudder::PrimitiveTypeClass::SignedInteger,
                    *size,
                )),
                boom::Size::Runtime(_) | boom::Size::Unknown => {
                    Arc::new(rudder::Type::ArbitraryLengthInteger)
                }
            },
            boom::Type::Bits { size } => match size {
                boom::Size::Static(size) => Arc::new(rudder::Type::new_primitive(
                    rudder::PrimitiveTypeClass::UnsignedInteger,
                    *size,
                )),
                boom::Size::Runtime(_) | boom::Size::Unknown => Arc::new(rudder::Type::Bits),
            },
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
            // register names kept for debugging
            registers: self.registers,
        }
    }
}

struct FunctionBuildContext<'ctx> {
    build_context: &'ctx mut BuildContext,
    rudder_fn: Function,
    blocks: HashMap<Id, rudder::Block>,
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
        trace!("resolving: {:x}", boom_block.id());

        if let Some(block) = self.blocks.get(&boom_block.id()) {
            trace!("already resolved: {:x}", boom_block.id());
            block.clone()
        } else {
            trace!("building: {:x}", boom_block.id());
            BlockBuildContext::new(self).build_block(boom_block)
        }
    }

    pub fn build_fn(&mut self, boom_fn: boom::FunctionDefinition) {
        trace!(
            "converting function {:?} from boom to rudder",
            boom_fn.signature.name
        );
        self.rudder_fn.inner.get_mut().entry_block = self.resolve_block(boom_fn.entry_block);
    }
}

struct BlockBuildContext<'ctx, 'fn_ctx> {
    function_build_context: &'fn_ctx mut FunctionBuildContext<'ctx>,
    builder: StatementBuilder,
    block: rudder::Block,
}

impl<'ctx: 'fn_ctx, 'fn_ctx> BlockBuildContext<'ctx, 'fn_ctx> {
    pub fn new(function_build_context: &'fn_ctx mut FunctionBuildContext<'ctx>) -> Self {
        let block = rudder::Block::new();

        Self {
            function_build_context,
            builder: StatementBuilder::new(block.weak()),
            block,
        }
    }

    fn ctx(&mut self) -> &mut BuildContext {
        self.function_build_context.build_context
    }

    fn fn_ctx(&mut self) -> &mut FunctionBuildContext<'ctx> {
        self.function_build_context
    }

    fn build_block(mut self, boom_block: boom::control_flow::ControlFlowBlock) -> rudder::Block {
        // pre-insert empty rudder block to avoid infinite recursion with cyclic blocks
        {
            let rudder_block = self.block.clone();
            self.fn_ctx().blocks.insert(boom_block.id(), rudder_block);
        }

        // convert statements
        boom_block
            .statements()
            .iter()
            .for_each(|stmt| self.build_statement(stmt.clone()));

        // check terminator, insert final rudder statement
        let kind = match boom_block.terminator() {
            boom::control_flow::Terminator::Return(value) => {
                let value = value.map(|v| self.build_value(Shared::new(v)));

                rudder::StatementKind::Return { value }
            }

            boom::control_flow::Terminator::Conditional {
                condition,
                target: boom_target,
                fallthrough: boom_fallthrough,
            } => {
                let condition = self.build_value(Shared::new(condition));
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
            boom::control_flow::Terminator::Panic(values) => {
                let values = values.iter().map(|v| self.build_value(v.clone())).collect();

                StatementKind::Panic(values)
            }
        };

        self.builder.build(kind);

        self.block.set_statements(self.builder.finish().into_iter());
        self.block
    }

    fn build_statement(&mut self, statement: Shared<boom::Statement>) {
        match &*statement.get() {
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
            boom::Statement::Panic(values) => {
                let statements = values
                    .iter()
                    .cloned()
                    .map(|v| self.build_value(v))
                    .collect();

                self.builder.build(StatementKind::Panic(statements));
            }
        }
    }

    fn build_copy(&mut self, value: Shared<boom::Value>, expression: &boom::Expression) {
        // ignore copies of undefined
        if let boom::Value::Literal(lit) = &*value.get() {
            if let boom::Literal::Undefined = *lit.get() {
                return;
            }
        }

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
                let target = match self.ctx().functions.get(name).cloned() {
                    Some((_, target, _)) => target,
                    // all functions should exist in boom by the time rudder is generated
                    None => {
                        panic!("unknown function {name}")
                    }
                };

                // cast all arguments to the correct type
                let casts = args
                    .iter()
                    .enumerate()
                    .map(|(i, stmt)| {
                        let typ = target.signature().1[i].typ();
                        self.builder.generate_cast(stmt.clone(), typ)
                    })
                    .collect::<Vec<_>>();

                // call statement
                self.builder.build(StatementKind::Call {
                    target,
                    args: casts.clone(),
                    tail: false,
                })
            }
        };

        if let Some(expression) = expression {
            self.build_expression_write(expression, fn_statement);
        }
    }

    /// Sail compiler builtin functions only!
    fn build_specialized_function(
        &mut self,
        name: InternedString,
        args: &[Statement],
    ) -> Option<Statement> {
        if Regex::new(r"eq_any<([a-zA-Z_]+)%>")
            .unwrap()
            .is_match(name.as_ref())
        {
            Some(self.builder.build(StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareEqual,
                lhs: args[0].clone(),
                rhs: args[1].clone(),
            }))
        } else {
            match name.as_ref() {
                "%i64->%i" => {
                    // lots of %i64->%i(Int(BigInt(-1))) so disabled this check
                   // assert_eq!(Type::s64(), *args[0].typ());
                    Some(
                    self.builder
                        .generate_cast(args[0].clone(), Arc::new(Type::ArbitraryLengthInteger)),
                )},

                "%i->%i64" => {
                    assert!(matches!(*args[0].typ(), Type::ArbitraryLengthInteger));

                    Some(
                        self.builder
                            .generate_cast(args[0].clone(), Arc::new(Type::s64())),
                    )
                }

                "make_the_value" | "size_itself_int" => Some(args[0].clone()),
                // %bv, %i, %i -> %bv
                "subrange_bits" => {
                    // end - start + 1
                    let one = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::s64()),
                        value: rudder::ConstantValue::SignedInteger(1),
                    });
                    let one = self.builder.generate_cast(one, args[1].typ());
                    let diff = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Sub,
                        lhs: args[1].clone(),
                        rhs: args[2].clone(),
                    });
                    let len = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: diff.clone(),
                        rhs: one.clone(),
                    });

                    Some(self.builder.build(StatementKind::BitExtract {
                        value: args[0].clone(),
                        start: args[2].clone(),
                        length: len,
                    }))
                }
                "eq_bits" | "eq_int" | "eq_bool" | "eq_string" => {
                    Some(self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::CompareEqual,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    }))
                }
                "neq_bits" | "neq_any<ESecurityState%>" => {
                    Some(self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::CompareNotEqual,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    }))
                }

                // val add_atom : (%i, %i) -> %i
                "add_atom" |
                  // val add_bits : (%bv, %bv) -> %bv
                "add_bits"

                => {
                    assert!(args[0].typ() == args[1].typ());
                    Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Add,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                }))},


                  // val add_bits_int : (%bv, %i) -> %bv
                  "add_bits_int" => {

                    let rhs = self
                        .builder
                        .generate_cast(args[1].clone(), Arc::new(Type::Bits));
                    Some(self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: args[0].clone(),
                        rhs,
                    }))

                }

                // val sub_bits_int : (%bv, %i) -> %bv
                "sub_bits_int" => {
                    let rhs = self
                        .builder
                        .generate_cast(args[1].clone(), Arc::new(Type::Bits));
                    Some(self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Sub,
                        lhs: args[0].clone(),
                        rhs,
                    }))
                }

                "sub_bits" | "sub_atom" => {
                    Some(self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Sub,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    }))
                }
                "mult_atom" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Multiply,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "tdiv_int" | "ediv_int" | "ediv_nat" => {
                    Some(self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Divide,
                        lhs: args[0].clone(),
                        rhs: args[1].clone(),
                    }))
                }

                "emod_nat" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Modulo,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "negate_atom" => Some(self.builder.build(StatementKind::UnaryOperation {
                    kind: UnaryOperationKind::Negate,
                    value: args[0].clone(),
                })),
                "abs_int_atom" => Some(self.builder.build(StatementKind::UnaryOperation {
                    kind: UnaryOperationKind::Absolute,
                    value: args[0].clone(),
                })),
                "min_int" => {
                    let true_value = args[0].clone();
                    let false_value = args[1].clone();

                    let condition = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::CompareLessThan,
                        lhs: true_value.clone(),
                        rhs: false_value.clone(),
                    });

                    Some(self.builder.build(StatementKind::Select {
                        condition,
                        true_value,
                        false_value,
                    }))
                }

                "max_int" => {
                    let true_value = args[0].clone();
                    let false_value = args[1].clone();

                    let condition = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::CompareGreaterThan,
                        lhs: true_value.clone(),
                        rhs: false_value.clone(),
                    });

                    Some(self.builder.build(StatementKind::Select {
                        condition,
                        true_value,
                        false_value,
                    }))
                }

                // val to_real : (%i) -> %real
                "to_real" => Some(
                    self.builder
                        .generate_cast(args[0].clone(), Arc::new(Type::f64())),
                ),

                "pow2" => Some(self.builder.build(StatementKind::UnaryOperation {
                    kind: UnaryOperationKind::Power2,
                    value: args[0].clone(),
                })),
                "lt_int" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::CompareLessThan,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "lteq_int" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::CompareLessThanOrEqual,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "gt_int" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::CompareGreaterThan,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "gteq_int" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::CompareGreaterThanOrEqual,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "not_vec" | "not_bool" => Some(self.builder.build(StatementKind::UnaryOperation {
                    kind: rudder::UnaryOperationKind::Not,
                    value: args[0].clone(),
                })),
                "and_vec" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::And,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "xor_vec" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Xor,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),
                "or_vec" => Some(self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Or,
                    lhs: args[0].clone(),
                    rhs: args[1].clone(),
                })),

                "sail_shiftright" | "_shr_int" => {
                    Some(self.builder.build(StatementKind::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftRight,
                        value: args[0].clone(),
                        amount: args[1].clone(),
                    }))
                }
                "sail_arith_shiftright" => {
                    Some(self.builder.build(StatementKind::ShiftOperation {
                        kind: ShiftOperationKind::ArithmeticShiftRight,
                        value: args[0].clone(),
                        amount: args[1].clone(),
                    }))
                }
                "sail_shiftleft" | "_shl_int" | "_shl8" | "_shl32" | "_shl1" => {
                    Some(self.builder.build(StatementKind::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftLeft,
                        value: args[0].clone(),
                        amount: args[1].clone(),
                    }))
                }

                "slice" => {
                    // uint64 n, uint64 start, uint64 len
                    Some(self.builder.build(StatementKind::BitExtract {
                        value: args[0].clone(),
                        start: args[1].clone(),
                        length: args[2].clone(),
                    }))
                }

                "plain_vector_access<o>"
                | "plain_vector_access<RBRBINFType>"
                | "plain_vector_access<RBRBSRCType>"
                | "plain_vector_access<RBRBTGTType>"
                | "plain_vector_access<RERRnFR_ElemType>"
                | "plain_vector_access<B64>"
                | "plain_vector_access<RPMEVTYPER_EL0_Type>" => {
                    Some(self.builder.build(StatementKind::ReadElement {
                        vector: args[0].clone(),
                        index: args[1].clone(),
                    }))
                }

                "bitvector_access" => {
                    let length = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(1),
                    });
                    let bitex = self.builder.build(StatementKind::BitExtract {
                        value: args[0].clone(),
                        start: args[1].clone(),
                        length,
                    });

                    Some(self.builder.generate_cast(bitex, Arc::new(Type::u1())))
                }

                "bitvector_length" => {
                    assert!(matches!(*args[0].typ(), Type::Bits));

                    Some(self.builder.build(StatementKind::SizeOf { value: args[0].clone() }))
                }

                "update_fbits" => {
                    //     if ((bit & 1) == 1) {
                    //         return op | (bit << n);
                    //    } else {
                    //         return op & ~(bit << n);
                    //    }
                    let op = self
                        .builder
                        .generate_cast(args[0].clone(), Arc::new(Type::u64()));
                    let n = args[1].clone();
                    let bit = self.builder.build(StatementKind::Cast {
                        kind: CastOperationKind::ZeroExtend,
                        typ: Arc::new(Type::u64()),
                        value: args[2].clone(),
                    });

                    // 1
                    let one = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(1),
                    });

                    // (bit & 1)
                    let and = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::And,
                        lhs: bit.clone(),
                        rhs: one.clone(),
                    });

                    //  (bit & 1) == 1
                    let condition = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::CompareEqual,
                        lhs: and,
                        rhs: one,
                    });

                    // bit << n
                    let shift = self.builder.build(StatementKind::ShiftOperation {
                        kind: ShiftOperationKind::LogicalShiftLeft,
                        value: bit.clone(),
                        amount: n,
                    });

                    // op | (bit << n)
                    let true_value = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Or,
                        lhs: op.clone(),
                        rhs: shift.clone(),
                    });

                    // ~(bit << n)
                    let inverse = self.builder.build(StatementKind::UnaryOperation {
                        kind: rudder::UnaryOperationKind::Complement,
                        value: shift,
                    });

                    // op & ~(bit << n)
                    let false_value = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::And,
                        lhs: op,
                        rhs: inverse,
                    });

                    Some(self.builder.build(StatementKind::Select {
                        condition,
                        true_value,
                        false_value,
                    }))
                }

                // %bv -> %i
                "UInt0" => {
                    // just copy bits

                    Some(self.builder.build(StatementKind::Cast { kind: CastOperationKind::ZeroExtend, typ:  Arc::new(Type::ArbitraryLengthInteger), value: args[0].clone()}))
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
                    Some(self.builder.build(StatementKind::Cast { kind: CastOperationKind::SignExtend, typ:  Arc::new(Type::ArbitraryLengthInteger), value:  args[0].clone()}))
                }





                 // val ZeroExtend0 : (%bv, %i) -> %bv
                // val sail_zero_extend : (%bv, %i) -> %bv
                "ZeroExtend0" | "sail_zero_extend" => {
                    Some(self.builder.build(StatementKind::BitsCast { kind: CastOperationKind::ZeroExtend, typ: Arc::new(Type::Bits), value: args[0].clone(), length: args[1].clone() }))
                }

                    // val SignExtend0 : (%bv, %i) -> %bv
                // val sail_sign_extend : (%bv, %i) -> %bv
                  "SignExtend0"
                | "sail_sign_extend" => {
                    Some(self.builder.build(StatementKind::BitsCast { kind: CastOperationKind::SignExtend, typ: Arc::new(Type::Bits), value: args[0].clone(), length: args[1].clone() }))
                }

    // val truncate : (%bv, %i) -> %bv
    "truncate" => {
        Some(self.builder.build(StatementKind::BitsCast { kind: CastOperationKind::Truncate, typ: Arc::new(Type::Bits), value: args[0].clone(), length: args[1].clone() }))
    }

                "sail_zeros" => {
                    let length = args[0].clone();

                    let _0 = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u8()),
                        value: rudder::ConstantValue::UnsignedInteger(0),
                    });

                    let value = self.builder.generate_cast(_0, Arc::new(Type::Bits));

                    Some(self.builder.build(StatementKind::BitsCast { kind: CastOperationKind::ZeroExtend, typ: Arc::new(Type::Bits), value, length }))


                }

                "sail_assert" => Some(self.builder.build(StatementKind::Assert {
                    condition: args[0].clone(),
                })),

                "write_gpr_from_vector" => {
                    // todo assert args[2] is always "GPRs"
                    // assuming GPRs are contiguoous

                    // %i argument to unsigned
                    let n = self
                        .builder
                        .generate_cast(args[0].clone(), Arc::new(Type::u64()));

                    let base = self.ctx().registers.get(&"R0".into()).unwrap().1;

                    let base = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(base),
                    });

                    let eight = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(8),
                    });

                    let offset = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Multiply,
                        lhs: n,
                        rhs: eight,
                    });

                    let offset = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: base,
                        rhs: offset,
                    });

                    Some(self.builder.build(StatementKind::WriteRegister {
                        offset,
                        value: args[1].clone(),
                    }))
                }
                "read_gpr_from_vector" => {
                    // todo assert args[1] is always "GPRs"
                    // assuming GPRs are contiguoous

                    // %i argument to unsigned
                    let n = self
                        .builder
                        .generate_cast(args[0].clone(), Arc::new(Type::u64()));

                    let base = self.ctx().registers.get(&"R0".into()).unwrap().1;

                    let base = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(base),
                    });

                    let eight = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u64()),
                        value: rudder::ConstantValue::UnsignedInteger(8),
                    });

                    let offset = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Multiply,
                        lhs: n,
                        rhs: eight,
                    });

                    let offset = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: base,
                        rhs: offset,
                    });

                    Some(self.builder.build(StatementKind::ReadRegister {
                        typ: Arc::new(Type::u64()),
                        offset,
                    }))
                }

                // val bitvector_update : (%bv, %i, %bit) -> %bv
                "bitvector_update" => {
                    let target = self
                        .builder
                        .generate_cast(args[0].clone(), Arc::new(Type::Bits));
                    let i = args[1].clone();
                    let bit = self
                        .builder
                        .generate_cast(args[0].clone(), Arc::new(Type::Bits));

                    let _1 = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u64()),
                        value: ConstantValue::UnsignedInteger(1),
                    });

                    Some(self.builder.build(StatementKind::BitInsert {
                        original_value: target,
                        insert_value: bit,
                        start: i,
                        length: _1,
                    }))

                }

                // val append_64 : (%bv, %bv64) -> %bv
                "append_64" => {

                    let rhs = self
                        .builder
                        .generate_cast(args[1].clone(), Arc::new(Type::Bits));
                    Some(self.generate_concat(args[0].clone(), rhs))

                }

                "bitvector_concat" => Some(self.generate_concat(args[0].clone(), args[1].clone())),
                "get_slice_int" => Some(self.builder.build(StatementKind::BitExtract {
                    value: args[1].clone(),
                    start: args[2].clone(),
                    length: args[0].clone(),
                })),

                "set_slice_bits" => {
                    //val set_slice_bits : (%i, %i, %bv, %i, %bv) -> %bv
                    // len, slen, x, pos, y
                    let _len = args[0].clone();
                    let slen = args[1].clone();
                    let destination = args[2].clone();
                    let start = args[3].clone();
                    let source = args[4].clone();

                    // destination[start..] = source[0..source.len()]

                    Some(self.generate_set_slice(destination, source, slen, start))
                }

                "update_subrange_bits" => {
                    let destination = args[0].clone();
                    let end = args[1].clone();
                    let start = args[2].clone();
                    let source = args[3].clone();

                    // copy source[0..(end - start + 1)] into dest[start..(end + 1)]

                    // let length = end - start
                    let source_length = self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Sub,
                        lhs: end,
                        rhs: start.clone(),
                    });

                    Some(self.generate_set_slice(destination, source, source_length, start))
                }

                "sail_branch_announce" => Some(self.builder.build(StatementKind::Constant {
                    typ: Arc::new(Type::unit()),
                    value: rudder::ConstantValue::Unit,
                })),

                "replicate_bits" => {
                    // // bundle length = bits_length * count
                    let count = self
                        .builder
                        .generate_cast(args[1].clone(), Arc::new(Type::u64()));
                    Some(self.builder.build(StatementKind::Call {
                        target: REPLICATE_BITS_BOREALIS_INTERNAL.clone(),
                        args: vec![args[0].clone(), count],
                        tail: false,
                    }))
                }

                /* ### NON-BUILTIN FUNCTIONS BELOW THIS POINT ### */
                /* To maintain correctness, borealis must only specialize on actual Sail compiler builtins, specializing other functions means restricting compatibiliy on a specific model, however memory access simply must be overwritten */
                "Mem_set" => {
                    // todo: check size is correct, maybe do something with access description
                    let address = args[0].clone();
                    let _size = args[1].clone();
                    let _accdesc = args[2].clone();
                    let value_in_name = args[3].clone();

                    Some(self.builder.build(StatementKind::WriteMemory {
                        offset: address,
                        value: value_in_name,
                    }))
                }
                "DataMemoryBarrier" => Some(self.builder.build(StatementKind::Constant {
                    typ: Arc::new(Type::unit()),
                    value: ConstantValue::Unit,
                })),
                "HaveEL" => {
                    let two = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::new_primitive(
                            rudder::PrimitiveTypeClass::UnsignedInteger,
                            2,
                        )),
                        value: ConstantValue::UnsignedInteger(2),
                    });
                    Some(self.builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::CompareLessThan,
                        lhs: args[0].clone(),
                        rhs: two,
                    }))
                    // el < 2
                }
                // ignore
                "append_str" => Some(args[0].clone()),
                "print_endline" | "AArch64_DC" => {
                    Some(self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::unit()),
                        value: ConstantValue::Unit,
                    }))
                }
                _ => None,
            }
        }
    }

    /// Generates rudder for a writing a statement to a boom::Expression
    fn build_expression_write(&mut self, target: &boom::Expression, source: Statement) {
        let idents = expression_field_collapse(target);
        let (root, fields) = idents
            .split_first()
            .expect("expression should always at least contain the root");

        match self.fn_ctx().rudder_fn.get_local_variable(*root) {
            Some(symbol) => {
                let (indices, outer_type) =
                    fields_to_indices(&self.ctx().structs, symbol.typ(), fields);

                let cast = self.builder.generate_cast(source, outer_type);

                self.builder.build(StatementKind::WriteVariable {
                    symbol,
                    indices,
                    value: cast,
                });
            }
            None => {
                //register lookup
                let Some((register_type, register_offset)) =
                    self.ctx().registers.get(root).cloned()
                else {
                    panic!("wtf is {root}");
                };

                let (field_offsets, outer_type) =
                    fields_to_offsets(&self.ctx().structs, register_type, fields);

                // offset + offset of each field
                let offset = register_offset + field_offsets.iter().sum::<usize>();

                // cast to outermost type
                let cast = self.builder.generate_cast(source, outer_type);

                let offset = self.builder.build(StatementKind::Constant {
                    typ: Arc::new(Type::u32()),
                    value: rudder::ConstantValue::UnsignedInteger(offset),
                });

                self.builder.build(StatementKind::WriteRegister {
                    offset,
                    value: cast,
                });
            }
        }
    }

    /// Last statement returned is the value
    fn build_value(&mut self, boom_value: Shared<boom::Value>) -> Statement {
        let (base, fields) = value_field_collapse(boom_value.clone());

        let borrow = base.get();

        match &*borrow {
            boom::Value::Identifier(ident) => {
                // local variable
                if let Some(symbol) = self.fn_ctx().rudder_fn.get_local_variable(*ident) {
                    let (indices, _) =
                        fields_to_indices(&self.ctx().structs, symbol.typ(), &fields);

                    return self
                        .builder
                        .build(StatementKind::ReadVariable { symbol, indices });
                }

                // parameter
                if let Some(symbol) = self.fn_ctx().rudder_fn.get_parameter(*ident) {
                    let (indices, _) =
                        fields_to_indices(&self.ctx().structs, symbol.typ(), &fields);

                    return self
                        .builder
                        .build(StatementKind::ReadVariable { symbol, indices });
                }

                // register
                if let Some((typ, register_offset)) = self.ctx().registers.get(ident).cloned() {
                    let (offsets, outer_type) =
                        fields_to_offsets(&self.ctx().structs, typ.clone(), &fields);

                    let offset = register_offset + offsets.iter().sum::<usize>();

                    let offset = self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u32()),
                        value: rudder::ConstantValue::UnsignedInteger(offset),
                    });

                    return self.builder.build(StatementKind::ReadRegister {
                        typ: outer_type,
                        offset,
                    });
                }

                // enum
                if let Some(value) = self
                    .ctx()
                    .enums
                    .iter()
                    .find_map(|(_, (_, variants))| variants.get(ident))
                    .cloned()
                {
                    return self.builder.build(StatementKind::Constant {
                        typ: Arc::new(Type::u32()),
                        value: rudder::ConstantValue::UnsignedInteger(value.try_into().unwrap()),
                    });
                }

                panic!("unknown ident: {:?}\n{:?}", ident, boom_value);
            }

            boom::Value::Literal(literal) => self.build_literal(&literal.get()),
            boom::Value::Operation(op) => self.build_operation(op),
            boom::Value::Struct { name, fields } => {
                let (typ, field_name_index_map) = self.ctx().structs.get(name).cloned().unwrap();

                let mut field_statements = vec![None; fields.len()];

                for NamedValue { name, value } in fields {
                    let field_statement = self.build_value(value.clone());
                    let idx = field_name_index_map.get(name).unwrap();
                    field_statements[*idx] = Some(field_statement);
                }

                self.builder.build(StatementKind::CreateComposite {
                    typ: typ.clone(),
                    fields: field_statements.into_iter().map(|o| o.unwrap()).collect(),
                })
            }

            boom::Value::Field { .. } => panic!("fields should have already been flattened"),

            boom::Value::CtorKind { .. } => todo!(),
            boom::Value::CtorUnwrap { .. } => todo!(),
        }
    }

    fn build_literal(&mut self, literal: &boom::Literal) -> Statement {
        let kind = match literal {
            boom::Literal::Int(i) => StatementKind::Constant {
                typ: Arc::new(Type::ArbitraryLengthInteger),
                value: rudder::ConstantValue::SignedInteger(
                    i.try_into().unwrap_or_else(|_| panic!("{i:x?}")),
                ),
            },
            boom::Literal::Bits(bits) => StatementKind::Constant {
                typ: Arc::new(Type::new_primitive(
                    rudder::PrimitiveTypeClass::UnsignedInteger,
                    bits.len(),
                )),
                value: rudder::ConstantValue::UnsignedInteger(
                    bits_to_int(bits).try_into().unwrap(),
                ),
            },
            boom::Literal::Bit(bit) => StatementKind::Constant {
                typ: Arc::new(Type::u1()),
                value: rudder::ConstantValue::UnsignedInteger(bit.value().try_into().unwrap()),
            },
            boom::Literal::Bool(b) => StatementKind::Constant {
                typ: Arc::new(Type::u1()),
                value: rudder::ConstantValue::UnsignedInteger(if *b { 1 } else { 0 }),
            },
            boom::Literal::String(str) => StatementKind::Constant {
                typ: Arc::new(Type::u32()),
                value: rudder::ConstantValue::UnsignedInteger(str.key().try_into().unwrap()),
            },
            boom::Literal::Unit => StatementKind::Constant {
                typ: Arc::new(Type::unit()),
                value: rudder::ConstantValue::Unit,
            },
            boom::Literal::Reference(_) => todo!(),
            boom::Literal::Undefined => unimplemented!(),
        };

        self.builder.build(kind)
    }

    fn build_operation(&mut self, op: &boom::Operation) -> Statement {
        match op {
            boom::Operation::Not(value) => {
                let value = self.build_value(value.clone());
                self.builder.build(StatementKind::UnaryOperation {
                    kind: rudder::UnaryOperationKind::Not,
                    value,
                })
            }
            boom::Operation::Complement(value) => {
                let value = self.build_value(value.clone());
                self.builder.build(StatementKind::UnaryOperation {
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
                    Type::Bits | Type::ArbitraryLengthInteger => todo!(),
                };

                self.builder.build(StatementKind::Cast {
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

                self.builder.build(StatementKind::ShiftOperation {
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

                    lhs = self
                        .builder
                        .generate_cast(lhs.clone(), destination_type.clone());
                    rhs = self.builder.generate_cast(rhs.clone(), destination_type);
                }

                self.builder.build(StatementKind::BinaryOperation {
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
                })
            }
        }
    }

    /// Copies source[0..source_length] into dest[start..start + source_length]
    fn generate_set_slice(
        &mut self,
        destination: Statement,
        source: Statement,
        source_length: Statement,
        destination_start_offset: Statement,
    ) -> Statement {
        // // let length = end - start
        // let length = self
        //     .statement_builder
        //     .build(StatementKind::BinaryOperation {
        //         kind: BinaryOperationKind::Sub,
        //         lhs: end,
        //         rhs: start.clone(),
        //     });

        // let source_mask = (1 << (length) - 1);
        let one = self.builder.build(StatementKind::Constant {
            typ: Arc::new(Type::u64()),
            value: rudder::ConstantValue::UnsignedInteger(1),
        });
        let one = self.builder.generate_cast(one, source.typ());
        let shifted = self.builder.build(StatementKind::ShiftOperation {
            kind: ShiftOperationKind::LogicalShiftLeft,
            value: one.clone(),
            amount: source_length,
        });
        let source_mask = self.builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::Sub,
            lhs: shifted,
            rhs: one,
        });

        // let masked_source = source & source_mask
        let masked_source = self.builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::And,
            lhs: source,
            rhs: source_mask.clone(),
        });

        // let source = masked_source << start
        let source = self.builder.build(StatementKind::ShiftOperation {
            kind: ShiftOperationKind::LogicalShiftLeft,
            value: masked_source,
            amount: destination_start_offset.clone(),
        });

        // let dest_mask = ~(source_mask << start)
        let shifted_source_mask = self.builder.build(StatementKind::ShiftOperation {
            kind: ShiftOperationKind::LogicalShiftLeft,
            value: source_mask,
            amount: destination_start_offset,
        });
        let destination_mask = self.builder.build(StatementKind::UnaryOperation {
            kind: rudder::UnaryOperationKind::Complement,
            value: shifted_source_mask,
        });

        // let dest = dest & dest_mask
        let destination = self.builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::And,
            lhs: destination,
            rhs: destination_mask,
        });

        // let result = source | dest
        self.builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::Or,
            lhs: destination,
            rhs: source,
        })
    }

    fn generate_concat(&mut self, lhs: Statement, rhs: Statement) -> Statement {
        match (&*lhs.typ(), &*rhs.typ()) {
            (Type::Bits, Type::Bits) => {
                let l_value = self
                    .builder
                    .generate_cast(lhs.clone(), Arc::new(Type::u128()));
                let l_length = self.builder.build(StatementKind::SizeOf { value: lhs });

                let r_value = self
                    .builder
                    .generate_cast(rhs.clone(), Arc::new(Type::u128()));
                let r_length = self.builder.build(StatementKind::SizeOf { value: rhs });

                let shift = self.builder.build(StatementKind::ShiftOperation {
                    kind: ShiftOperationKind::LogicalShiftLeft,
                    value: l_value,
                    amount: r_length.clone(),
                });

                let value = self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Or,
                    lhs: shift,
                    rhs: r_value,
                });
                let length = self.builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Add,
                    lhs: l_length,
                    rhs: r_length,
                });

                // lhs.value << rhs.len | rhs.value
                // lhs.len + rhs.len
                self.builder
                    .build(StatementKind::CreateBits { value, length })
            }
            _ => todo!(),
        }
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
                current_expression = &expression;
            }
            boom::Expression::Address(_) => panic!("addresses not supported"),
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
    structs: &HashMap<InternedString, (Arc<Type>, HashMap<InternedString, usize>)>,
    initial_type: Arc<Type>,
    fields: &[InternedString],
) -> (Vec<usize>, Arc<Type>) {
    let mut current_type = initial_type;

    let mut indices = vec![];

    fields.iter().for_each(|field| {
        // get the fields of the current struct
        let (_, (struct_typ, fields)) = structs
            .iter()
            .find(|(_, (candidate, _))| Arc::ptr_eq(&current_type, candidate))
            .expect("failed to find struct :(");

        // get index and push
        let idx = *fields.get(field).unwrap();
        indices.push(idx);

        // update current struct to point to field
        let Type::Composite(fields) = &**struct_typ else {
            panic!("cannot get fields of non-composite")
        };
        current_type = fields[idx].clone();
    });

    (indices, current_type)
}

fn fields_to_offsets(
    structs: &HashMap<InternedString, (Arc<Type>, HashMap<InternedString, usize>)>,
    initial_type: Arc<Type>,
    fields: &[InternedString],
) -> (Vec<usize>, Arc<Type>) {
    let mut current_type = initial_type;

    let mut offsets = vec![];

    fields.iter().for_each(|field| {
        // get the fields of the current struct
        let (_, (_, fields)) = structs
            .iter()
            .find(|(_, (candidate, _))| Arc::ptr_eq(&current_type, candidate))
            .expect("failed to find struct :(");

        // get index and push
        let idx = *fields.get(field).unwrap();
        offsets.push(current_type.byte_offset(idx).unwrap());

        // update current struct to point to field
        let Type::Composite(fields) = &*current_type else {
            panic!("cannot get fields of non-composite")
        };
        current_type = fields[idx].clone();
    });

    (offsets, current_type)
}

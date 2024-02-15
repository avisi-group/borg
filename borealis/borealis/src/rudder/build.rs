use {
    crate::{
        boom,
        rudder::{
            self, Context, Function, FunctionKind, Statement, StatementInner, StatementKind,
            Symbol, Type, ValueKind,
        },
    },
    common::{intern::InternedString, HashMap},
    std::{cell::RefCell, rc::Rc},
};

pub fn from_boom(ast: &boom::Ast) -> Context {
    let mut build_ctx = BuildContext::default();

    // DEFINITION ORDER DEPENDANT!!!
    ast.definitions.iter().for_each(|def| match def {
        boom::Definition::Enum { name, variants } => build_ctx.add_enum(*name, variants),
        boom::Definition::Union { name, fields } => build_ctx.add_union(*name, fields),
        boom::Definition::Struct { name, fields } => build_ctx.add_struct(*name, fields),
        boom::Definition::Pragma { key, value } => (),
        boom::Definition::Let { bindings, body } => (),
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
    /// Name of struct maps to the rudder type and a map of field names to field indices
    structs: HashMap<InternedString, (Rc<rudder::Type>, HashMap<InternedString, usize>)>,
    /// Name of union maps to the rudder type and a map of field names to field indices
    unions: HashMap<InternedString, (Rc<rudder::Type>, HashMap<InternedString, usize>)>,
    /// Name of enum maps to the rudder type and a map of enum variants to the integer discriminant of that variant
    enums: HashMap<InternedString, (Rc<rudder::Type>, HashMap<InternedString, u32>)>,

    /// Functions
    functions: HashMap<InternedString, (FunctionKind, Function, boom::FunctionDefinition)>,
}

impl BuildContext {
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

        let fields = fields
            .iter()
            .enumerate()
            .map(|(idx, boom::NamedType { name, .. })| (*name, idx))
            .collect();

        if self.unions.insert(name, (typ, fields)).is_some() {
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
                    self.resolve_type(definition.signature.return_type.clone()),
                    definition
                        .signature
                        .parameters
                        .borrow()
                        .iter()
                        .map(|boom::Parameter { typ, .. }| self.resolve_type(typ.clone()))
                        .collect(),
                ),
                definition.clone(),
            ),
        );
    }

    fn build_functions(&mut self) {
        self.functions
            .clone()
            .into_iter()
            .filter(|(name, _)| {
                *name == InternedString::new("integer_arithmetic_addsub_immediate_decode")
            })
            .for_each(|(_, (kind, rudder_fn, boom_fn))| {
                FunctionBuildContext::new(self).build_fn(rudder_fn.clone(), boom_fn.clone());
            });
    }

    fn build_statement(&self, statement: Rc<RefCell<boom::Statement>>) -> Vec<rudder::Statement> {
        match &*statement.borrow() {
            boom::Statement::TypeDeclaration { name, typ } => todo!(),
            boom::Statement::Copy { expression, value } => todo!(),
            boom::Statement::FunctionCall {
                expression,
                name,
                arguments,
            } => todo!(),
            boom::Statement::Label(_) => todo!(),
            boom::Statement::Goto(_) => todo!(),
            boom::Statement::Jump { .. } => todo!(),
            boom::Statement::End(_) => todo!(),
            boom::Statement::Undefined => todo!(),
            boom::Statement::If { .. } => todo!(),
            boom::Statement::Exit(_) => todo!(),
            boom::Statement::Comment(_) => todo!(),
        }
    }

    fn resolve_type(&self, typ: Rc<RefCell<boom::Type>>) -> Rc<rudder::Type> {
        match &*typ.borrow() {
            boom::Type::Unit => Rc::new(rudder::Type::void()),
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
                    boom::Size::Runtime(_) | boom::Size::Unknown => 64, // todo blame tom when this inevitably breaks
                };

                Rc::new(rudder::Type::new_primitive(tc, element_width))
            }
            boom::Type::Constant(_) => todo!(),
            boom::Type::Enum { name, .. } => self.enums.get(name).unwrap().0.clone(),
            boom::Type::Union { name, .. } => self.unions.get(name).unwrap().0.clone(),
            boom::Type::Struct { name, .. } => self.structs.get(name).unwrap().0.clone(),
            boom::Type::List { element_type } => todo!(),
            boom::Type::Vector { element_type } => todo!(),
            boom::Type::FixedVector {
                length,
                element_type,
            } => todo!(),
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
        }
    }
}

struct FunctionBuildContext<'ctx> {
    symbols: HashMap<InternedString, Symbol>,
    build_context: &'ctx mut BuildContext,
}

impl<'ctx> FunctionBuildContext<'ctx> {
    pub fn new(build_context: &'ctx mut BuildContext) -> Self {
        Self {
            symbols: HashMap::default(),
            build_context,
        }
    }

    pub fn build_fn(&mut self, rudder_fn: Function, boom_fn: boom::FunctionDefinition) {
        rudder_fn.inner.borrow_mut().entry_block =
            BlockBuildContext::new(self).build_block(boom_fn.entry_block.clone());
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

    fn resolve_value(&self, value: Rc<RefCell<boom::Value>>) -> rudder::Value {
        rudder::Value {
            kind: ValueKind::Statement(Statement {
                inner: Rc::new(RefCell::new(StatementInner {
                    name: "test".into(),
                    kind: rudder::StatementKind::Constant {
                        value: rudder::ConstantValue::UnsignedInteger(5),
                    },
                })),
            }),
        }
    }

    fn build_block(&mut self, boom_block: boom::control_flow::ControlFlowBlock) -> rudder::Block {
        println!("building block");
        let rudder_block = rudder::Block::new();

        // convert statements
        // rudder_block.set_statements(
        //     boom_block
        //         .statements()
        //         .into_iter()
        //         .map(|stmt| self.build_statement(stmt)),
        // );

        // check terminator, insert final rudder statement
        rudder_block.add_statement(match boom_block.terminator() {
            boom::control_flow::Terminator::Return(value) => rudder::StatementKind::Return {
                value: value.map(|v| self.resolve_value(Rc::new(RefCell::new(v)))),
            },

            boom::control_flow::Terminator::Conditional {
                condition,
                target: boom_target,
                fallthrough: boom_fallthrough,
            } => {
                let condition = self.resolve_value(Rc::new(RefCell::new(condition)));

                let rudder_true_target =
                    BlockBuildContext::new(self.fn_ctx()).build_block(boom_target);
                let rudder_false_target =
                    BlockBuildContext::new(self.fn_ctx()).build_block(boom_fallthrough);

                StatementKind::Branch {
                    condition,
                    true_target: rudder_true_target,
                    false_target: rudder_false_target,
                }
            }
            boom::control_flow::Terminator::Unconditional {
                target: boom_target,
            } => {
                let rudder_target = BlockBuildContext::new(self.fn_ctx()).build_block(boom_target);
                StatementKind::Jump {
                    target: rudder_target,
                }
            }
        });

        rudder_block
    }

    fn build_statement(&mut self) {}
}

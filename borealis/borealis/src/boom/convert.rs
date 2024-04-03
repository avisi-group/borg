//! JIB to BOOM conversion

use {
    crate::boom::{
        self, control_flow::build_graph, Bit, FunctionSignature, NamedType, Parameter, Size,
    },
    common::{intern::InternedString, HashMap},
    sailrs::{jib_ast, sail_ast},
    std::{borrow::Borrow, cell::RefCell, rc::Rc},
};

type Parameters = Vec<Rc<RefCell<boom::Type>>>;
type Return = Rc<RefCell<boom::Type>>;

/// Consumes JIB AST and produces BOOM
#[derive(Debug, Default)]
pub struct BoomEmitter {
    /// BOOM AST being constructed by walker
    ast: boom::Ast,
    /// Temporarily stored type signatures as spec and function definitions are
    /// separate
    function_types: HashMap<InternedString, (Parameters, Return)>,
}

impl BoomEmitter {
    /// Create a new `BoomEmitter`
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a sequence of JIB definitions
    pub fn process<I: IntoIterator<Item = jib_ast::Definition>>(&mut self, definitions: I) {
        definitions
            .into_iter()
            .for_each(|def| self.process_definition(&def));
    }

    /// Emit BOOM AST
    pub fn finish(self) -> boom::Ast {
        self.ast
    }

    fn process_definition(&mut self, definition: &jib_ast::Definition) {
        match definition {
            jib_ast::Definition::Register(ident, typ, _) => {
                self.ast
                    .registers
                    .insert(ident.as_interned(), convert_type(typ));
            }
            jib_ast::Definition::Type(type_def) => {
                let def = match type_def {
                    jib_ast::TypeDefinition::Enum(name, variants) => boom::Definition::Enum {
                        name: name.as_interned(),
                        variants: convert_variants(variants.as_ref()),
                    },
                    jib_ast::TypeDefinition::Struct(name, fields) => boom::Definition::Struct {
                        name: name.as_interned(),
                        fields: convert_fields(fields.iter()),
                    },
                    jib_ast::TypeDefinition::Variant(name, fields) => boom::Definition::Union {
                        name: name.as_interned(),
                        fields: convert_fields(fields.iter()),
                    },
                };
                self.ast.definitions.push(def);
            }
            jib_ast::Definition::Let(_, bindings, body) => {
                self.ast.definitions.push(boom::Definition::Let {
                    bindings: bindings
                        .iter()
                        .map(|(ident, typ)| NamedType {
                            name: ident.as_interned(),
                            typ: convert_type(typ),
                        })
                        .collect(),
                    body: convert_body(body.as_ref()),
                });
            }
            jib_ast::Definition::Val(id, _, parameters, out) => {
                self.function_types.insert(
                    id.as_interned(),
                    (
                        parameters.iter().map(convert_type).collect(),
                        convert_type(out),
                    ),
                );
            }
            jib_ast::Definition::Fundef(name, _, arguments, body) => {
                let (parameter_types, return_type) =
                    self.function_types.remove(&name.as_interned()).unwrap();

                let parameters = Rc::new(RefCell::new(
                    arguments
                        .iter()
                        .map(sail_ast::Identifier::as_interned)
                        .zip(parameter_types)
                        .map(|(name, typ)| Parameter {
                            name,
                            typ,
                            is_ref: false,
                        })
                        .collect::<Vec<_>>(),
                ));

                let name = name.as_interned();

                let body = convert_body(body.as_ref());

                //debug!("building new control flow graph for {name}");
                let control_flow = build_graph(&body);

                self.ast.functions.insert(
                    name,
                    boom::FunctionDefinition {
                        signature: FunctionSignature {
                            name,
                            parameters,
                            return_type,
                        },
                        entry_block: control_flow,
                    },
                );
            }
            jib_ast::Definition::Startup(_, _) => todo!(),
            jib_ast::Definition::Finish(_, _) => todo!(),
            &jib_ast::Definition::Pragma(key, value) => self
                .ast
                .definitions
                .push(boom::Definition::Pragma { key, value }),
        };
    }
}

fn convert_type<T: Borrow<jib_ast::Type>>(typ: T) -> Rc<RefCell<boom::Type>> {
    Rc::new(RefCell::new(match typ.borrow() {
        jib_ast::Type::Lbits => boom::Type::Int {
            signed: false,
            size: Size::Unknown,
        },
        jib_ast::Type::Fbits(i) => boom::Type::Int {
            signed: false,
            size: Size::Static(usize::try_from(*i).unwrap()),
        },

        jib_ast::Type::Lint => boom::Type::Int {
            signed: true,
            size: Size::Unknown,
        },
        jib_ast::Type::Fint(i) => boom::Type::Int {
            signed: true,
            size: Size::Static(usize::try_from(*i).unwrap()),
        },

        jib_ast::Type::Unit => boom::Type::Unit,
        jib_ast::Type::Bool => boom::Type::Bool,
        jib_ast::Type::Bit => boom::Type::Bit,
        jib_ast::Type::String => boom::Type::String,
        jib_ast::Type::Real => boom::Type::Real,

        jib_ast::Type::Enum(name, variants) => boom::Type::Enum {
            name: name.as_interned(),
            variants: convert_variants(variants.as_ref()),
        },
        jib_ast::Type::Struct(name, fields) => boom::Type::Struct {
            name: name.as_interned(),
            fields: convert_fields(fields.as_ref()),
        },
        jib_ast::Type::Variant(name, fields) => boom::Type::Union {
            name: name.as_interned(),
            fields: convert_fields(fields.as_ref()),
        },
        jib_ast::Type::Fvector(length, typ) => boom::Type::FixedVector {
            length: *length,
            element_type: convert_type(&**typ),
        },
        jib_ast::Type::Vector(typ) => boom::Type::Vector {
            element_type: (convert_type(&**typ)),
        },
        jib_ast::Type::List(typ) => boom::Type::List {
            element_type: (convert_type(&**typ)),
        },
        jib_ast::Type::Ref(typ) => boom::Type::Reference(convert_type(&**typ)),
        jib_ast::Type::Constant(_)
        | jib_ast::Type::Sbits(_)
        | jib_ast::Type::Float(_)
        | jib_ast::Type::RoundingMode
        | jib_ast::Type::Tup(_)
        | jib_ast::Type::Poly(_) => todo!(),
    }))
}

fn convert_body(instructions: &[jib_ast::Instruction]) -> Vec<Rc<RefCell<boom::Statement>>> {
    instructions
        .iter()
        .flat_map(|instr| convert_statement(&instr.inner))
        .collect()
}

fn convert_statement(statement: &jib_ast::InstructionAux) -> Vec<Rc<RefCell<boom::Statement>>> {
    if let jib_ast::InstructionAux::Block(instructions) = statement {
        return convert_body(instructions.as_ref());
    }

    let statements = match statement {
        jib_ast::InstructionAux::Decl(typ, name) => vec![boom::Statement::TypeDeclaration {
            name: convert_name(name),
            typ: convert_type(typ),
        }],
        jib_ast::InstructionAux::Init(typ, name, value) => {
            vec![
                boom::Statement::TypeDeclaration {
                    name: convert_name(name),
                    typ: convert_type(typ),
                },
                boom::Statement::Copy {
                    expression: boom::Expression::Identifier(convert_name(name)),
                    value: convert_value(value),
                },
            ]
        }
        jib_ast::InstructionAux::Jump(condition, target) => vec![boom::Statement::Jump {
            condition: convert_value(condition),
            target: *target,
        }],
        jib_ast::InstructionAux::Goto(s) => vec![boom::Statement::Goto(*s)],
        jib_ast::InstructionAux::Label(s) => vec![boom::Statement::Label(*s)],
        jib_ast::InstructionAux::Funcall(expression, _, (name, _), args) => {
            vec![boom::Statement::FunctionCall {
                expression: convert_expression(expression),
                name: name.as_interned(),
                arguments: args.iter().map(convert_value).collect(),
            }]
        }
        jib_ast::InstructionAux::Copy(expression, value) => vec![boom::Statement::Copy {
            expression: convert_expression(expression).unwrap(),
            value: convert_value(value),
        }],
        jib_ast::InstructionAux::Clear(_, _) => vec![],
        jib_ast::InstructionAux::Undefined(_) => vec![boom::Statement::Undefined],
        jib_ast::InstructionAux::Exit(s) => vec![boom::Statement::Exit(*s)],
        jib_ast::InstructionAux::End(name) => vec![boom::Statement::End(convert_name(name))],
        jib_ast::InstructionAux::If(condition, if_body, else_body, _) => {
            vec![boom::Statement::If {
                condition: convert_value(condition),
                if_body: convert_body(if_body.as_ref()),
                else_body: convert_body(else_body.as_ref()),
            }]
        }

        jib_ast::InstructionAux::TryBlock(_) => vec![],
        jib_ast::InstructionAux::Throw(value) => {
            vec![boom::Statement::Panic(vec![convert_value(value)])]
        }
        jib_ast::InstructionAux::Comment(s) => vec![boom::Statement::Comment(*s)],
        jib_ast::InstructionAux::Block(..) => unimplemented!(),
        jib_ast::InstructionAux::Raw(_) => todo!(),
        jib_ast::InstructionAux::Return(_) => todo!(),
        jib_ast::InstructionAux::Reset(_, _) => todo!(),
        jib_ast::InstructionAux::Reinit(_, _, _) => todo!(),
    };

    statements
        .into_iter()
        .map(RefCell::new)
        .map(Rc::new)
        .collect()
}

fn convert_name(name: &jib_ast::Name) -> InternedString {
    match name {
        jib_ast::Name::Name(ident, _) | jib_ast::Name::Global(ident, _) => ident.as_interned(),
        jib_ast::Name::HaveException(_) | jib_ast::Name::CurrentException(_) => {
            InternedString::from_static("exception")
        }
        jib_ast::Name::ThrowLocation(_) => InternedString::from_static("throw"),
        jib_ast::Name::Return(_) => InternedString::from_static("return"),
    }
}

fn convert_expression(expression: &jib_ast::Expression) -> Option<boom::Expression> {
    match expression {
        jib_ast::Expression::Id(name, _) => Some(boom::Expression::Identifier(convert_name(name))),
        jib_ast::Expression::Rmw(_, _, _) => todo!(),
        jib_ast::Expression::Field(expression, ident) => Some(boom::Expression::Field {
            expression: Box::new(convert_expression(expression).unwrap()),
            field: ident.as_interned(),
        }),
        jib_ast::Expression::Addr(expr) => Some(boom::Expression::Address(Box::new(
            convert_expression(expr).unwrap(),
        ))),
        jib_ast::Expression::Tuple(_, _) => todo!(),
        jib_ast::Expression::Void => None,
    }
}

fn convert_value(value: &jib_ast::Value) -> Rc<RefCell<boom::Value>> {
    Rc::new(RefCell::new(match value {
        jib_ast::Value::Id(name, _) => boom::Value::Identifier(convert_name(name)),
        jib_ast::Value::Lit(vl, _) => boom::Value::Literal(convert_literal(vl)),
        jib_ast::Value::Tuple(_, _) => todo!(),
        jib_ast::Value::Struct(fields, jib_ast::Type::Struct(ident, _)) => boom::Value::Struct {
            name: ident.as_interned(),
            fields: fields
                .iter()
                .map(|(ident, value)| boom::NamedValue {
                    name: ident.as_interned(),
                    value: convert_value(value),
                })
                .collect(),
        },
        jib_ast::Value::Struct(_, _) => panic!("encountered struct with non-struct type"),
        jib_ast::Value::CtorKind(value, (ctor, unifiers), _) => boom::Value::CtorKind {
            value: (convert_value(value)),
            identifier: ctor.as_interned(),
            types: unifiers.iter().map(convert_type).collect(),
        },
        jib_ast::Value::CtorUnwrap(value, (ctor, unifiers), _) => boom::Value::CtorUnwrap {
            value: (convert_value(value)),
            identifier: ctor.as_interned(),
            types: unifiers.iter().map(convert_type).collect(),
        },
        jib_ast::Value::TupleMember(_, _, _) => todo!(),
        jib_ast::Value::Call(op, values) => {
            let values = values.iter().map(convert_value).collect::<Vec<_>>();

            let op = match op {
                jib_ast::Op::Bnot => boom::Operation::Not(values[0].clone()),
                jib_ast::Op::Bor => todo!(),
                jib_ast::Op::Band => todo!(),
                jib_ast::Op::ListHead => todo!(),
                jib_ast::Op::ListTail => todo!(),
                jib_ast::Op::Eq => todo!(),
                jib_ast::Op::Neq => {
                    boom::Operation::Not(Rc::new(RefCell::new(boom::Value::Operation(
                        boom::Operation::Equal(values[0].clone(), values[1].clone()),
                    ))))
                }
                jib_ast::Op::Ilt => boom::Operation::LessThan(values[0].clone(), values[1].clone()),

                jib_ast::Op::Ilteq => todo!(),
                jib_ast::Op::Igt => {
                    boom::Operation::GreaterThan(values[0].clone(), values[1].clone())
                }
                jib_ast::Op::Igteq => todo!(),
                jib_ast::Op::Iadd => boom::Operation::Add(values[0].clone(), values[1].clone()),
                jib_ast::Op::Isub => {
                    boom::Operation::Subtract(values[0].clone(), values[1].clone())
                }
                jib_ast::Op::Unsigned(_) => todo!(),
                jib_ast::Op::Signed(_) => todo!(),
                jib_ast::Op::Bvnot => todo!(),
                jib_ast::Op::Bvor => todo!(),
                jib_ast::Op::Bvand => todo!(),
                jib_ast::Op::Bvxor => todo!(),
                jib_ast::Op::Bvadd => todo!(),
                jib_ast::Op::Bvsub => todo!(),
                jib_ast::Op::Bvaccess => todo!(),
                jib_ast::Op::Concat => todo!(),
                jib_ast::Op::ZeroExtend(_) => todo!(),
                jib_ast::Op::SignExtend(_) => todo!(),
                jib_ast::Op::Slice(_) => todo!(),
                jib_ast::Op::Sslice(_) => todo!(),
                jib_ast::Op::SetSlice => todo!(),
                jib_ast::Op::Replicate(_) => todo!(),
                jib_ast::Op::ListIsEmpty => todo!(),
            };
            boom::Value::Operation(op)
        }
        jib_ast::Value::Field(value, ident) => boom::Value::Field {
            value: (convert_value(value)),
            field_name: ident.as_interned(),
        },
    }))
}

fn convert_literal(literal: &jib_ast::Vl) -> Rc<RefCell<boom::Literal>> {
    Rc::new(RefCell::new(match literal {
        jib_ast::Vl::Bits(bits) => {
            // todo: this may need a `.rev`
            // update 2024-03-21: turns out it does on sail17arm94
            boom::Literal::Bits(bits.iter().rev().map(convert_bit).collect())
        }
        jib_ast::Vl::Bit(bit) => boom::Literal::Bit(convert_bit(bit)),
        jib_ast::Vl::Bool(b) => boom::Literal::Bool(*b),
        jib_ast::Vl::Unit => boom::Literal::Unit,
        jib_ast::Vl::Int(bigint) => boom::Literal::Int(bigint.0.clone()),
        jib_ast::Vl::String(s) => boom::Literal::String(*s),
        jib_ast::Vl::Real(_) => todo!(),
        jib_ast::Vl::Enum(_) => todo!(),
        jib_ast::Vl::Ref(s) => boom::Literal::Reference(*s),
        jib_ast::Vl::Undefined => boom::Literal::Undefined,
    }))
}

fn convert_bit(bit: &jib_ast::BitU) -> boom::Bit {
    match bit {
        jib_ast::BitU::B0 => Bit::Zero,
        jib_ast::BitU::B1 => Bit::One,
        jib_ast::BitU::BU => Bit::Unknown,
    }
}

/// Converts variants of an enum from JIB to BOOM
fn convert_variants(variants: &[sail_ast::Identifier]) -> Vec<InternedString> {
    variants
        .iter()
        .map(sail_ast::Identifier::as_interned)
        .collect()
}

/// Converts fields of a struct or union from JIB to BOOM
///
/// Generics are required to be able to convert from `LinkedList<((Identifier,
/// LinkedList<Type>), Box<Type>)>` *and* `LinkedList<((Identifier,
/// LinkedList<Type>), Type)>`.
fn convert_fields<
    'a,
    TYPE: Borrow<jib_ast::Type> + 'a,
    ITER: IntoIterator<Item = &'a (sail_ast::Identifier, TYPE)>,
>(
    fields: ITER,
) -> Vec<NamedType> {
    fields
        .into_iter()
        .map(|(name, typ)| NamedType {
            name: name.as_interned(),
            typ: convert_type(typ.borrow()),
        })
        .collect()
}

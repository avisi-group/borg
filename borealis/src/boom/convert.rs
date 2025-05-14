//! JIB to BOOM conversion

use {
    crate::{
        boom::{
            self, Bit, FunctionDefinition, FunctionSignature, Literal, NamedType, Parameter, Size,
            Type, control_flow::ControlFlowBlock,
        },
        shared::Shared,
    },
    common::{hashmap::HashMap, intern::InternedString},
    isla_lib::{
        bitvector::{BV, b64::B64},
        ir::{Def, Exp, Instr, Loc, Ty},
    },
    itertools::Itertools,
    num_bigint::BigInt,
    std::{borrow::Borrow, collections::BTreeMap, hash::Hash},
};

type Parameters = Vec<Shared<boom::Type>>;
type Return = Shared<boom::Type>;

/// Consumes JIB AST and produces BOOM
#[derive(Debug, Default)]
pub struct BoomEmitter {
    /// BOOM AST being constructed by walker
    ast: boom::Ast,
    /// Temporarily stored type signatures as spec and function definitions are
    /// separate
    function_types: HashMap<InternedString, (Parameters, Return)>,
    /// Register initialization statements (also letbinds)
    register_init_statements: Vec<Shared<boom::Statement>>,
}

impl BoomEmitter {
    /// Create a new `BoomEmitter`
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a sequence of JIB definitions
    /// IntoParallelIterator
    pub fn process<I: IntoIterator<Item = Def<InternedString, B64>>>(&mut self, definitions: I) {
        definitions
            .into_iter() //.into_par_iter
            .for_each(|def| self.process_definition(&def));
    }

    /// Emit BOOM AST
    pub fn finish(mut self) -> boom::Ast {
        // create register initialization function
        {
            let entry_block = ControlFlowBlock::new();

            entry_block.set_statements(self.register_init_statements);
            entry_block.set_terminator(boom::control_flow::Terminator::Return(None));

            self.ast.functions.insert(
                "borealis_register_init".into(),
                FunctionDefinition {
                    signature: FunctionSignature {
                        name: "borealis_register_init".into(),
                        parameters: Shared::new(vec![]),
                        return_type: None,
                    },
                    entry_block,
                },
            );
        }

        // external functions
        // todo: handle this better from IR
        self.ast.functions.extend(self.function_types.iter().map(
            |(name, (parameters, return_type))| {
                (
                    *name,
                    FunctionDefinition {
                        signature: FunctionSignature {
                            name: *name,
                            parameters: Shared::new(
                                parameters
                                    .iter()
                                    .enumerate()
                                    .map(|(i, typ)| Parameter {
                                        name: format!("p{i}").into(),
                                        typ: typ.clone(),
                                    })
                                    .collect(),
                            ),
                            return_type: Some(return_type.clone()),
                        },
                        entry_block: ControlFlowBlock::new(),
                    },
                )
            },
        ));

        self.ast
    }

    fn process_definition(&mut self, definition: &Def<InternedString, B64>) {
        match definition {
            Def::Register(ident, typ, body) => {
                self.ast.registers.insert(*ident, self.convert_type(typ));
                let mut statements = body
                    .iter()
                    .flat_map(|i| self.convert_instruction(i))
                    .collect::<Vec<_>>();
                self.register_init_statements.append(&mut statements);
            }
            Def::Enum(name, variants) => {
                self.ast.enums.insert(*name, variants.clone());
            }
            Def::Struct(name, fields) => {
                let fields = self.convert_fields(fields.iter());
                self.ast.structs.insert(*name, fields);
            }
            Def::Union(name, fields) => {
                let fields = self.convert_fields(fields.iter());
                self.ast.unions.insert(*name, fields);
            }
            Def::Let(bindings, body) => {
                bindings.iter().for_each(|(ident, typ)| {
                    self.ast.registers.insert(*ident, self.convert_type(typ));
                });
                let mut statements = body
                    .iter()
                    .flat_map(|i| self.convert_instruction(i))
                    .collect::<Vec<_>>();
                self.register_init_statements.append(&mut statements);
            }
            Def::Extern(id, _, _, parameters, out) | Def::Val(id, parameters, out) => {
                self.function_types.insert(
                    *id,
                    (
                        parameters.iter().map(|t| self.convert_type(t)).collect(),
                        self.convert_type(out),
                    ),
                );
            }
            Def::Fn(name, arguments, body) => {
                let (parameter_types, return_type) = self.function_types.remove(name).unwrap();

                let parameters = Shared::new(
                    arguments
                        .iter()
                        .copied()
                        .zip(parameter_types)
                        .map(|(name, typ)| Parameter { name, typ })
                        .collect::<Vec<_>>(),
                );

                let entry_block = self.convert_body(body.as_ref());

                // // make implicit return variable explicit
                let mut statements = entry_block.statements();
                statements.insert(
                    0,
                    Shared::new(boom::Statement::VariableDeclaration {
                        name: "return".into(),
                        typ: return_type.clone(),
                    }),
                );
                entry_block.set_statements(statements);

                self.ast.functions.insert(
                    *name,
                    boom::FunctionDefinition {
                        signature: FunctionSignature {
                            name: *name,
                            parameters,
                            return_type: Some(return_type),
                        },
                        entry_block,
                    },
                );
            }
            Def::Pragma(key, value) => {
                self.ast.pragmas.insert(
                    InternedString::from(key.as_str()),
                    InternedString::from(value.as_str()),
                );
            }

            Def::Files(items) => log::warn!("files: {items:?}"),
        };
    }

    /// Converts fields of a struct or union from JIB to BOOM
    ///
    /// Generics are required to be able to convert from
    /// `LinkedList<((Identifier, LinkedList<Type>), Box<Type>)>` *and*
    /// `LinkedList<((Identifier, LinkedList<Type>), Type)>`.
    fn convert_fields<
        'a,
        TYPE: Borrow<Ty<InternedString>> + 'a,
        ITER: IntoIterator<Item = &'a (InternedString, TYPE)>,
    >(
        &self,
        fields: ITER,
    ) -> Vec<NamedType> {
        fields
            .into_iter()
            .map(|(name, typ)| NamedType {
                name: *name,
                typ: self.convert_type(typ.borrow()),
            })
            .collect()
    }

    fn convert_type<T: Borrow<Ty<InternedString>>>(&self, typ: T) -> Shared<boom::Type> {
        Shared::new(match typ.borrow() {
            Ty::I64 => boom::Type::Integer {
                size: Size::Static(64),
            },
            Ty::I128 => boom::Type::Integer {
                size: Size::Static(128),
            },
            Ty::AnyBits => boom::Type::Bits {
                size: Size::Unknown,
            },
            Ty::Bits(width) => boom::Type::Bits {
                size: Size::Static(usize::try_from(*width).unwrap()),
            },
            Ty::Float(fpty) => boom::Type::Float, // todo: properly handle floating point type

            Ty::Unit => boom::Type::Unit,
            Ty::Bool => boom::Type::Bool,
            Ty::Bit => boom::Type::Bit,
            Ty::String => boom::Type::String,
            Ty::Real => boom::Type::Real,
            Ty::RoundingMode => boom::Type::RoundingMode,

            Ty::FixedVector(length, ty) => boom::Type::FixedVector {
                length: usize::try_from(*length).unwrap(),
                element_type: self.convert_type(&**ty),
            },
            Ty::Vector(ty) | Ty::List(ty) => boom::Type::Vector {
                element_type: (self.convert_type(&**ty)),
            },
            Ty::Ref(ty) => self.convert_type(&**ty).get().clone(),

            // enums are constants
            Ty::Enum(_) => boom::Type::Integer {
                size: Size::Static(32),
            },
            Ty::Struct(name) => boom::Type::Struct {
                name: *name,
                fields: self.ast.structs.get(name).unwrap().clone(),
            },
            Ty::Union(name) => boom::Type::Union {
                name: *name,
                fields: self.ast.unions.get(name).unwrap().clone(),
            },
        })
    }
    fn convert_body(&self, instructions: &[Instr<InternedString, B64>]) -> ControlFlowBlock {
        let entry = ControlFlowBlock::new();

        let mut current_block = entry.clone();
        let mut iter = instructions.iter().enumerate();
        let mut block_locations = BTreeMap::<usize, ControlFlowBlock>::new();

        let mut current_statements = vec![];

        // for every instruction in the body
        while let Some((idx, instr)) = iter.next() {
            // if the current index was the target of a jump, start a new block
            if let Some(next_block) = block_locations.get(&idx) {
                current_block.set_statements(current_statements.clone());
                current_statements.clear();

                current_block.set_terminator(boom::control_flow::Terminator::Unconditional {
                    target: next_block.clone(),
                });
                next_block.add_parent(&current_block);

                current_block = ControlFlowBlock::new();
            }

            match instr {
                // unconditional jump
                Instr::Goto(target) => {
                    let target_block = block_locations
                        .entry(*target)
                        .or_insert_with(ControlFlowBlock::new);

                    current_block.set_statements(current_statements.clone());
                    current_statements.clear();

                    current_block.set_terminator(boom::control_flow::Terminator::Unconditional {
                        target: target_block.clone(),
                    });
                    target_block.add_parent(&current_block);

                    current_block = ControlFlowBlock::new();
                }

                // conditional jump
                Instr::Jump(condition, target, _) => {
                    let fallthrough_block = ControlFlowBlock::new();

                    let target_block = block_locations
                        .entry(*target)
                        .or_insert_with(ControlFlowBlock::new);

                    current_block.set_statements(current_statements.clone());
                    current_statements.clear();

                    current_block.set_terminator(boom::control_flow::Terminator::Conditional {
                        condition: convert_expression(condition).get().clone(),
                        target: target_block.clone(),
                        fallthrough: fallthrough_block.clone(),
                    });
                    target_block.add_parent(&current_block);

                    current_block = fallthrough_block;
                }
                // return
                Instr::End => {
                    current_block.set_statements(current_statements.clone());
                    current_statements.clear();

                    current_block.set_terminator(boom::control_flow::Terminator::Return(Some(
                        boom::Value::Identifier("return".into()),
                    )));

                    current_block = ControlFlowBlock::new();
                }
                // panic
                Instr::Exit(cause, _) => {
                    current_block.set_statements(current_statements.clone());
                    current_statements.clear();

                    current_block.set_terminator(boom::control_flow::Terminator::Panic(
                        boom::Value::Literal(Shared::new(boom::Literal::String(
                            format!("{cause:?}").into(),
                        ))),
                    ));

                    current_block = ControlFlowBlock::new();
                }
                _ => current_statements.extend_from_slice(&self.convert_instruction(instr)),
            }
        }

        entry
    }

    fn convert_instruction(
        &self,
        instr: &Instr<InternedString, B64>,
    ) -> Vec<Shared<boom::Statement>> {
        // jib_ast::InstructionAux::Decl(typ, name) =>

        // jib_ast::InstructionAux::Init(typ, name, value) =>

        // jib_ast::InstructionAux::Jump(condition, target) =>
        // vec![boom::Statement::Jump {     condition: convert_value(condition),
        //     target: *target,
        // }],
        // jib_ast::InstructionAux::Goto(s) => vec![boom::Statement::Goto(*s)],
        // jib_ast::InstructionAux::Label(s) => vec![boom::Statement::Label(*s)],
        // jib_ast::InstructionAux::Funcall(ret, _, (name, _), args) => {
        //     let CReturn::One(expression) = ret else {
        //         todo!()
        //     };
        //     vec![boom::Statement::FunctionCall {
        //         expression: convert_expression(expression),
        //         name: name.as_interned(),
        //         arguments: args.iter().map(convert_value).collect(),
        //     }]
        // }
        // jib_ast::InstructionAux::Copy(expression, value) =>
        // vec![boom::Statement::Copy {     expression:
        // convert_expression(expression).unwrap(),     value: convert_value(value),
        // }],
        // jib_ast::InstructionAux::Clear(_, _) => vec![],
        // jib_ast::InstructionAux::Undefined(_) => vec![boom::Statement::Undefined],
        // jib_ast::InstructionAux::Exit(s) => vec![boom::Statement::Exit(*s)],
        // jib_ast::InstructionAux::End(name) =>
        // vec![boom::Statement::End(convert_name(name))],
        // jib_ast::InstructionAux::If(condition, if_body, else_body, _) => {
        //     vec![boom::Statement::If {
        //         condition: convert_value(condition),
        //         if_body: convert_body(if_body.as_ref()),
        //         else_body: convert_body(else_body.as_ref()),
        //     }]
        // }

        // jib_ast::InstructionAux::Throw(value) => {
        //     vec![boom::Statement::Panic(convert_value(value))]
        // }
        // jib_ast::InstructionAux::Comment(s) => vec![boom::Statement::Comment(*s)],

        let statements = match instr {
            Instr::Decl(name, ty, _) => vec![boom::Statement::VariableDeclaration {
                name: *name,
                typ: self.convert_type(ty),
            }],
            Instr::Init(name, ty, exp, _) => {
                vec![
                    boom::Statement::VariableDeclaration {
                        name: *name,
                        typ: self.convert_type(ty),
                    },
                    boom::Statement::Copy {
                        expression: boom::Expression::Identifier(*name),
                        value: convert_expression(exp),
                    },
                ]
            }

            Instr::Copy(loc, exp, _) => {
                vec![boom::Statement::Copy {
                    expression: convert_location(loc),
                    value: convert_expression(exp),
                }]
            }
            Instr::Monomorphize(_, source_loc) => todo!(),
            Instr::Call(loc, _, name, args, source_loc) => {
                let expression = convert_location(loc);
                vec![boom::Statement::FunctionCall {
                    expression: Some(expression),
                    name: *name,
                    arguments: args.iter().map(convert_expression).collect(),
                }]
            }
            Instr::PrimopUnary(loc, _, exp, source_loc) => todo!(),
            Instr::PrimopBinary(loc, _, exp, exp1, source_loc) => todo!(),
            Instr::PrimopVariadic(loc, _, exps, source_loc) => todo!(),
            Instr::PrimopReset(loc, _, source_loc) => todo!(),

            Instr::Arbitrary => vec![boom::Statement::Undefined],

            Instr::Jump(..) => unreachable!("jump"),
            Instr::Goto(_) => unreachable!("goto"),
            Instr::Exit(..) => unreachable!("exit"),
            Instr::End => unreachable!("end"),
        };

        statements.into_iter().map(Shared::new).collect()
    }
}

// fn convert_name(name: &jib_ast::Name) -> InternedString {
//     match name {
//         jib_ast::Name::Name(ident, _) => ident.as_interned(),
//         jib_ast::Name::HaveException(_) =>
// InternedString::from_static("have_exception"),
//         jib_ast::Name::CurrentException(_) =>
// InternedString::from_static("current_exception"),
//         jib_ast::Name::ThrowLocation(_) =>
// InternedString::from_static("throw"),         jib_ast::Name::Return(_) =>
// InternedString::from_static("return"),         jib_ast::Name::Channel(_, _)
// => InternedString::from_static("channel"),     }
// }

// jib_ast::Expression::Id(name, _) =>
// Some(boom::Expression::Identifier(convert_name(name))),
// jib_ast::Expression::Rmw(_, _, _) => todo!(),
// jib_ast::Expression::Field(expression, ident) =>

// jib_ast::Expression::Addr(expr) =>
// Some(boom::Expression::Address(Box::new(
//     convert_expression(expr).unwrap(),
// ))),
// jib_ast::Expression::Tuple(_, _) => todo!(),
// jib_ast::Expression::Void => None,

fn convert_location(location: &Loc<InternedString>) -> boom::Expression {
    match location {
        Loc::Id(id) => boom::Expression::Identifier(*id),
        Loc::Field(loc, field) => boom::Expression::Field {
            expression: Box::new(convert_location(loc)),
            field: *field,
        },
        Loc::Addr(loc) => boom::Expression::Address(Box::new(convert_location(loc))),
    }
}

fn convert_expression(expression: &Exp<InternedString>) -> Shared<boom::Value> {
    Shared::new(match expression {
        Exp::Id(id) => boom::Value::Identifier(*id),
        Exp::Ref(id) => boom::Value::Identifier(*id), /* todo: fix this */
        Exp::Bool(b) => boom::Value::Literal(Shared::new(boom::Literal::Bool(*b))),
        Exp::Bits(bits) => boom::Value::Literal(Shared::new(boom::Literal::Bits(
            // todo: use bits type in rest of codebase
            bits.to_vec()
                .into_iter()
                .map(|b| if b { boom::Bit::One } else { boom::Bit::Zero })
                .collect(),
        ))),
        Exp::String(s) => {
            boom::Value::Literal(Shared::new(boom::Literal::String(s.as_str().into())))
        }
        Exp::Unit => boom::Value::Literal(Shared::new(boom::Literal::Unit)),
        Exp::I64(i) => boom::Value::Literal(Shared::new(boom::Literal::Int(BigInt::from(*i)))),
        Exp::I128(i) => boom::Value::Literal(Shared::new(boom::Literal::Int(BigInt::from(*i)))),
        Exp::Undefined(ty) => boom::Value::Literal(Shared::new(boom::Literal::Undefined)), /* todo: use type somehow? */
        Exp::Struct(name, fields) => boom::Value::Struct {
            name: *name,
            fields: fields
                .iter()
                .map(|(ident, value)| boom::NamedValue {
                    name: *ident,
                    value: convert_expression(value),
                })
                .collect(),
        },
        Exp::Kind(name, exp) => boom::Value::CtorKind {
            value: (convert_expression(exp)),
            identifier: *name,
        },
        Exp::Unwrap(name, exp) => boom::Value::CtorUnwrap {
            value: (convert_expression(exp)),
            identifier: *name,
        },
        Exp::Field(exp, field) => boom::Value::Field {
            value: convert_expression(exp),
            field_name: *field,
        },
        Exp::Call(op, values) => {
            let values = values.iter().map(convert_expression).collect::<Vec<_>>();

            let op = match op {
                isla_lib::ir::Op::Not => boom::Operation::Not(values[0].clone()),
                isla_lib::ir::Op::Or => todo!(),
                isla_lib::ir::Op::And => todo!(),
                isla_lib::ir::Op::Eq => todo!(),
                isla_lib::ir::Op::Neq => boom::Operation::Not(Shared::new(boom::Value::Operation(
                    boom::Operation::Equal(values[0].clone(), values[1].clone()),
                ))),
                isla_lib::ir::Op::Lteq => todo!(),
                isla_lib::ir::Op::Lt => {
                    boom::Operation::LessThan(values[0].clone(), values[1].clone())
                }
                isla_lib::ir::Op::Gteq => todo!(),
                isla_lib::ir::Op::Gt => {
                    boom::Operation::GreaterThan(values[0].clone(), values[1].clone())
                }
                isla_lib::ir::Op::Add => boom::Operation::Add(values[0].clone(), values[1].clone()),
                isla_lib::ir::Op::Sub => {
                    boom::Operation::Subtract(values[0].clone(), values[1].clone())
                }
                isla_lib::ir::Op::Slice(_) => todo!(),
                isla_lib::ir::Op::SetSlice => todo!(),
                isla_lib::ir::Op::Signed(_) => todo!(),
                isla_lib::ir::Op::Unsigned(_) => todo!(),
                isla_lib::ir::Op::ZeroExtend(_) => todo!(),
                isla_lib::ir::Op::Bvnot => todo!(),
                isla_lib::ir::Op::Bvor => todo!(),
                isla_lib::ir::Op::Bvxor => todo!(),
                isla_lib::ir::Op::Bvand => todo!(),
                isla_lib::ir::Op::Bvadd => todo!(),
                isla_lib::ir::Op::Bvsub => todo!(),
                isla_lib::ir::Op::Bvaccess => todo!(),
                isla_lib::ir::Op::Concat => todo!(),
                isla_lib::ir::Op::Head => todo!(),
                isla_lib::ir::Op::Tail => todo!(),
                isla_lib::ir::Op::IsEmpty => todo!(),
            };

            boom::Value::Operation(op)
        }
    })
}

// fn convert_value(value: &jib_ast::Value) -> Shared<boom::Value> {
//     Shared::new(match value {
//         jib_ast::Value::Id(name, _) =>
// boom::Value::Identifier(convert_name(name)),         jib_ast::Value::Lit(vl,
// _) => boom::Value::Literal(convert_literal(vl)),
//         jib_ast::Value::Tuple(_, _) => todo!(),
//         jib_ast::Value::Struct(fields, jib_ast::Type::Struct(ident, _)) =>
// boom::Value::Struct {             name: ident.as_interned(),
//             fields: fields
//                 .iter()
//                 .map(|(ident, value)| boom::NamedValue {
//                     name: ident.as_interned(),
//                     value: convert_value(value),
//                 })
//                 .collect(),
//         },
//         jib_ast::Value::Struct(_, _) => panic!("encountered struct with
// non-struct type"),         jib_ast::Value::CtorKind(value, (ctor, unifiers),
// _) => boom::Value::CtorKind {             value: (convert_value(value)),
//             identifier: ctor.as_interned(),
//             types: unifiers.iter().map(convert_type).collect(),
//         },
//         jib_ast::Value::CtorUnwrap(value, (ctor, unifiers), _) =>
// boom::Value::CtorUnwrap {             value: (convert_value(value)),
//             identifier: ctor.as_interned(),
//             types: unifiers.iter().map(convert_type).collect(),
//         },
//         jib_ast::Value::TupleMember(_, _, _) => todo!(),
//         jib_ast::Value::Call(op, values) => {

//         }
//         jib_ast::Value::Field(value, ident) => boom::Value::Field {
//             value: (convert_value(value)),
//             field_name: ident.as_interned(),
//         },
//         // convert enum members into their indices as integer literals
//         jib_ast::Value::Member(ident, typ) => {
//             let jib_ast::Type::Enum(_, members) = typ else {
//                 todo!()
//             };

//             let member_index = members
//                 .iter()
//                 .find_position(|s| s.as_interned() == ident.as_interned())
//                 .unwrap_or_else(|| {
//                     panic!("failed to find index for enum {ident:?} of type
// {typ:?}")                 })
//                 .0;

//
// boom::Value::Literal(Shared::new(boom::Literal::Int(member_index.into())))
//         }
//     })
// }

// fn convert_literal(literal: &jib_ast::Vl) -> Shared<boom::Literal> {
//     Shared::new(match literal {
//         jib_ast::Vl::Bits(bits) => {
//             // this may need a `.rev`
//             // update 2024-03-21: turns out it does on sail17arm94
//             boom::Literal::Bits(bits.iter().rev().map(convert_bit).collect())
//         }
//         jib_ast::Vl::Bit(bit) => boom::Literal::Bit(convert_bit(bit)),
//         jib_ast::Vl::Bool(b) => boom::Literal::Bool(*b),
//         jib_ast::Vl::Unit => boom::Literal::Unit,
//         jib_ast::Vl::Int(bigint) => boom::Literal::Int(bigint.0.clone()),
//         jib_ast::Vl::String(s) => boom::Literal::String(*s),
//         jib_ast::Vl::Real(_) => todo!(),
//         jib_ast::Vl::Enum(_) => todo!(),
//         jib_ast::Vl::Ref(s) => boom::Literal::Reference(*s),
//         jib_ast::Vl::Undefined => boom::Literal::Undefined,
//     })
// }

// fn convert_bit(bit: BitU) -> boom::Bit {
//     match bit {
//         jib_ast::BitU::B0 => Bit::Zero,
//         jib_ast::BitU::B1 => Bit::One,
//         jib_ast::BitU::BU => Bit::Unknown,
//     }
// }

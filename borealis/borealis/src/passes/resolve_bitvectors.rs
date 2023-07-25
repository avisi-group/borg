//! JIB contains bitvectors GenC doesn't, need to represent them somehow.
//!
//! Inspection of all values of bitvector lengths in the ARMv8 model shows:
//!
//! * Minimum: 1
//! * Maximum: 64
//! * Mean: 2.88
//! * Mode: 1
//!
//! So almost always a tiny value, and all fit in a `uint64`, but length must be
//! stored too somehow. Possibly in the high 64 bits of a uint128?
//!
//! Length is known at compile time, and eq_vec is a builtin function, so could
//! pass in length values then?
//!
//! Maybe length doesn't matter, initially trying to replace them all with
//! uint64s
//!
//! new plan: try to determine the lenght of every non fixed bv at compile time

use crate::boom::Value;
use {
    crate::{
        boom::{visitor::Visitor, Ast, Bit, Expression, Literal, NamedType, Statement, Type},
        passes::{any::AnyExt, Pass},
    },
    common::{intern::InternedString, HashMap},
    num_bigint::BigInt,
    std::{cell::RefCell, rc::Rc},
};

enum Length {
    Variable(Rc<RefCell<Type>>),
    Fixed(isize),
}

pub struct ReplaceBitvectors {
    did_change: bool,
    /// bitvector lengths of local variables in a function, `None` represents
    /// not-yet-calculated lengths
    lengths: HashMap<InternedString, Length>,
}

impl ReplaceBitvectors {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {
            did_change: false,
            lengths: HashMap::default(),
        })
    }
}

impl Pass for ReplaceBitvectors {
    fn name(&self) -> &'static str {
        "ReplaceBitvectors"
    }

    fn reset(&mut self) {
        self.did_change = false;
        self.lengths.clear();
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|func| {
                self.reset();

                // insert any bitvector function parameters
                self.lengths
                    .extend(func.signature.parameters.iter().filter_map(
                        |NamedType { name, typ }| match &*typ.borrow() {
                            Type::FixedBits(length, _) => Some((*name, Length::Fixed(*length))),
                            Type::LargeBits(_) => {
                                log::warn!("unknown length bitvector in function signature");
                                None
                            }
                            _ => None,
                        },
                    ));

                self.visit_function_definition(func);

                self.did_change
            })
            .any()
    }
}

impl Visitor for ReplaceBitvectors {
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        // if a type decl for a bv add it to a list of bv's to be made constant

        match &*node.borrow() {
            Statement::TypeDeclaration { name, typ } => {
                match &*typ.borrow() {
                    // if we encounter a fixed variable,
                    Type::FixedBits(length, _) => {
                        println!("typedecl {name} fixed {length}");
                        self.lengths.insert(*name, Length::Fixed(*length));
                    }
                    Type::LargeBits(_) => {
                        println!("typedecl {name} variable");
                        self.lengths.insert(*name, Length::Variable(typ.clone()));
                    }
                    _ => {}
                }
            }
            Statement::Copy { expression, value } => {
                let Expression::Identifier(dest) = expression else {
                    return;
                };

                println!("copy into {dest}");

                // if the identifier being copied into is a variable bitvector, try and resolve
                // it's length
                if let Some(Length::Variable(typ)) = self.lengths.get(dest) {
                    match value {
                        // if the value is an identifier
                        Value::Identifier(source) => {
                            // and that identifier has a known length
                            if let Some(Length::Fixed(length)) = self.lengths.get(&source) {
                                // the source variable must also have that length
                                *typ.borrow_mut() = Type::FixedBits(*length, false);
                                self.lengths.insert(*dest, Length::Fixed(*length));
                            }
                        }

                        Value::Literal(literal) => {
                            let literal = &mut *literal.borrow_mut();
                            if let Literal::Bits(bits) = literal {
                                *typ.borrow_mut() = Type::FixedBits(bits.len() as isize, false);
                                self.lengths
                                    .insert(*dest, Length::Fixed(bits.len() as isize));

                                // replace bits with constant int
                                *literal = Literal::Int(BigInt::from(bits.iter().rev().fold(
                                    0,
                                    |acc, bit| {
                                        acc << 1
                                            | match bit {
                                                Bit::_0 => 0,
                                                Bit::_1 => 1,
                                                Bit::Unknown => panic!(),
                                            }
                                    },
                                )));
                            }
                        }

                        _ => (),
                    }
                }
            }
            _ => {}
        }

        // use assignments to determine length
        // function calls are special:/ need to think about handling these
    }
}

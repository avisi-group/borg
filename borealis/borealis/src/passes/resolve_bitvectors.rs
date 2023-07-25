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
//!
//! Some bv's might be impossible (e.g. imm length is runtime dependant) but
//! hoping as long as these don't involve concatenation we don't need to know
//! the length

use {
    crate::{
        boom::{
            control_flow::ControlFlowBlock,
            visitor::{Visitor, Walkable},
            Ast, Bit, Expression, FunctionDefinition, Literal, NamedType, Statement, Type, Value,
        },
        passes::{any::AnyExt, Pass},
    },
    common::{intern::InternedString, HashMap},
    itertools::Itertools,
    num_bigint::BigInt,
    once_cell::sync::Lazy,
    std::{cell::RefCell, rc::Rc},
};

pub struct ResolveBitvectors {
    did_change: bool,
    /// bitvector lengths of local variables in a function
    lengths: HashMap<InternedString, Length>,
    current_func: Option<FunctionDefinition>,
}

/// Length of a bitvector local variable
enum Length {
    /// Variable, and a reference to the type so we can modify it when the
    /// length is known
    Variable(Rc<RefCell<Type>>),
    /// Fixed, and the length
    Fixed(isize),
}

impl ResolveBitvectors {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {
            did_change: false,
            lengths: HashMap::default(),
            current_func: None,
        })
    }

    /// Resolves the length of a local variable
    fn resolve(&mut self, ident: InternedString, length: isize) {
        let Some(Length::Variable(typ)) = self.lengths.get(&ident) else {
            panic!("called resolve on non-variable bitvector");
        };

        *typ.borrow_mut() = Type::FixedBits(length, false);
        self.lengths.insert(ident, Length::Fixed(length));
    }

    /// Adds a bitvector type declaration to the mapping
    fn add_type_declaration(&mut self, name: InternedString, typ: Rc<RefCell<Type>>) {
        match &*typ.borrow() {
            // if we encounter a fixed variable,
            Type::FixedBits(length, _) => {
                self.lengths.insert(name, Length::Fixed(*length));
            }
            Type::LargeBits(_) => {
                self.lengths.insert(name, Length::Variable(typ.clone()));
            }
            _ => {}
        }
    }

    /// Try to use the value being assigned to a bitvector to determine it's
    /// length
    fn resolve_from_copy(&mut self, expression: &Expression, value: &Value) {
        let Expression::Identifier(dest) = expression else {
            return;
        };

        // if the identifier being copied into is a variable bitvector, try and resolve
        // it's length
        if let Some(Length::Variable(_)) = self.lengths.get(dest) {
            match value {
                // if the value is an identifier
                Value::Identifier(source) => {
                    // and that identifier has a known length
                    if let Some(Length::Fixed(length)) = self.lengths.get(&source) {
                        // the source variable must also have that length
                        self.resolve(*dest, *length);
                    }
                }

                Value::Literal(literal) => {
                    let literal = &mut *literal.borrow_mut();
                    if let Literal::Bits(bits) = literal {
                        self.resolve(*dest, bits.len() as isize);

                        // replace bits with constant int
                        *literal =
                            Literal::Int(BigInt::from(bits.iter().rev().fold(0, |acc, bit| {
                                acc << 1
                                    | match bit {
                                        Bit::_0 => 0,
                                        Bit::_1 => 1,
                                        Bit::Unknown => panic!(),
                                    }
                            })));
                    }
                }

                _ => (),
            }
        }
    }

    /// Resolves bitvectors in a function call
    ///
    /// If the function is a builtin bitvector operation (`eq_vec`, `Zeros`,
    /// etc), replace it with the corresponding logic.
    ///
    /// If the function has variable bitvector parameters, use arguments
    /// supplied to a function call to generate a monomorphised version of that
    /// function with fixed bitvector paramaters.
    fn resolve_fn(
        &mut self,
        statement: Rc<RefCell<Statement>>,
        expression: &Expression,
        name: InternedString,
        arguments: &[Value],
    ) {
        type HandlerFunction =
            fn(&mut ResolveBitvectors, Rc<RefCell<Statement>>, &Expression, &[Value]);

        // function handlers
        const HANDLERS: Lazy<HashMap<InternedString, HandlerFunction>> = Lazy::new(|| {
            let mappings = [("Zeros", zeros_handler as HandlerFunction)]
                .into_iter()
                .map(|(s, f)| (InternedString::from_static(s), f));

            HashMap::from_iter(mappings)
        });

        // execute function handler if the function call is to a builtin bitvector
        // function
        if let Some(handler) = HANDLERS.get(&name) {
            handler(self, statement.clone(), expression, arguments);
        }
    }
}

impl Pass for ResolveBitvectors {
    fn name(&self) -> &'static str {
        "ReplaceBitvectors"
    }

    fn reset(&mut self) {
        self.did_change = false;
        self.lengths.clear();
        self.current_func = None;
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

impl Visitor for ResolveBitvectors {
    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        self.current_func = Some(node.clone());
        node.walk(self);
    }

    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        let statement = {
            let borrow = node.borrow();
            borrow.clone()
        };
        match statement {
            Statement::TypeDeclaration { name, typ } => {
                self.add_type_declaration(name, typ.clone())
            }
            Statement::Copy { expression, value } => self.resolve_from_copy(&expression, &value),
            Statement::FunctionCall {
                expression: Some(expression),
                name,
                arguments,
            } => self.resolve_fn(node.clone(), &expression, name, &arguments),
            _ => {}
        }

        // use assignments to determine length
        // function calls are special:/ need to think about handling these
    }
}

fn zeros_handler(
    celf: &mut ResolveBitvectors,
    statement: Rc<RefCell<Statement>>,
    expression: &Expression,
    arguments: &[Value],
) {
    // get assignment to argument to Zeros
    assert_eq!(arguments.len(), 1);
    let Value::Identifier(ident) = arguments[0] else {
        panic!();
    };
    let Some(value) = get_assignment(
        celf.current_func.as_ref().unwrap().entry_block.clone(),
        ident,
    ) else {
        panic!("{ident}");
    };

    let Value::Literal(literal) = value else {
        panic!();
    };

    let Literal::Int(length) = &*literal.borrow() else {
        panic!();
    };

    // change type of destination to length
    let Expression::Identifier(destination) = expression else {
        panic!();
    };

    celf.resolve(*destination, isize::try_from(length).unwrap());

    // assign literal 0

    *statement.borrow_mut() = Statement::Copy {
        expression: expression.clone(),
        value: Value::Literal(Rc::new(RefCell::new(Literal::Int(0.into())))),
    }
}

fn get_assignment(entry_block: ControlFlowBlock, ident: InternedString) -> Option<Value> {
    entry_block
        .iter()
        .flat_map(|cfb| cfb.statements())
        .filter_map(|statement| {
            let res = {
                let borrow = statement.borrow();
                match &*borrow {
                    Statement::Copy { expression, value } => {
                        Some((expression.clone(), value.clone()))
                    }
                    _ => None,
                }
            };

            res
        })
        .filter_map(|(expr, value)| {
            let Expression::Identifier(assign) = expr else {
                return None;
            };

            if assign == ident {
                Some(value)
            } else {
                None
            }
        })
        .at_most_one()
        .unwrap()
}

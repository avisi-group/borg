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

use {
    crate::{
        boom::{
            control_flow::ControlFlowBlock,
            visitor::{Visitor, Walkable},
            Ast, Bit, Literal, Type,
        },
        passes::{any::AnyExt, Pass},
    },
    common::HashSet,
    num_bigint::BigInt,
    std::{cell::RefCell, rc::Rc},
};

pub struct ReplaceBitvectors {
    did_change: bool,
    visited_blocks: HashSet<ControlFlowBlock>,
}

impl ReplaceBitvectors {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {
            did_change: false,
            visited_blocks: HashSet::default(),
        })
    }
}

impl Pass for ReplaceBitvectors {
    fn name(&self) -> &'static str {
        "ReplaceBitvectors"
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|def| {
                self.did_change = false;
                self.visit_function_definition(def);
                self.did_change
            })
            .any()
    }
}

impl Visitor for ReplaceBitvectors {
    fn visit_type(&mut self, node: Rc<RefCell<Type>>) {
        {
            let node = &mut *node.borrow_mut();
            if let Type::Lbits(_) = node {
                // replace type with uint64
                *node = Type::Fint(64);
            }
        }
        node.walk(self);
    }

    fn visit_literal(&mut self, node: Rc<RefCell<Literal>>) {
        {
            let node = &mut *node.borrow_mut();
            if let Literal::Bits(bitvec) = node {
                // replace bits with constant int
                *node = Literal::Int(BigInt::from(bitvec.iter().rev().fold(0, |acc, bit| {
                    acc << 1
                        | match bit {
                            Bit::_0 => 0,
                            Bit::_1 => 1,
                            Bit::Unknown => panic!(),
                        }
                })));
            }
        }
        node.walk(self);
    }

    fn is_block_visited(&mut self, block: &ControlFlowBlock) -> bool {
        self.visited_blocks.contains(block)
    }

    fn set_block_visited(&mut self, block: &ControlFlowBlock) {
        self.visited_blocks.insert(block.clone());
    }
}

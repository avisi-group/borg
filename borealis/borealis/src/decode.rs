//! Instruction decoding

use sail::{
    ast::{
        FunctionClause, IdentifierAux, LiteralAux, NumericExpression, NumericExpressionAux,
        Pattern, PatternAux, PatternMatch, PatternMatchAux, TypArgAux, TypAux, L,
    },
    types::OCamlString,
    visitor::Visitor,
    visitor::Walkable,
};

#[derive(Debug)]
enum DecodeBit {
    Zero,
    One,
    Unknown,
}

/// Visitor for building instruction decode strings
pub struct DecodeStringVisitor {
    in_clause: bool,
    instruction: Vec<DecodeBit>,
}

impl DecodeStringVisitor {
    /// Create a new empty instance
    pub fn new() -> Self {
        Self {
            in_clause: false,
            instruction: Vec::new(),
        }
    }
}

impl Visitor for DecodeStringVisitor {
    fn visit_function_clause(&mut self, node: &FunctionClause) {
        if node.inner.identifier.inner
            == IdentifierAux::Identifier(OCamlString::String("decode64".to_owned()))
        {
            match &node.annotation.0 {
                L::Range(p1, _) => {
                    dbg!(p1);
                }
                _ => (),
            }
            println!("starting ferdia decode64");
            self.in_clause = true;
            self.instruction = vec![];
            node.walk(self);
            println!("{:?}", self.instruction);
            self.in_clause = false;
            println!("end ferdia decode64");
        }
    }

    fn visit_pattern_match(&mut self, node: &PatternMatch) {
        match &node.inner {
            PatternMatchAux::Expression(_, _) => todo!(),
            PatternMatchAux::When(_pattern, _, _body) => todo!(),
        }
    }

    fn visit_pattern(&mut self, node: &Pattern) {
        if !self.in_clause {
            return;
        }

        match &*node.inner {
            PatternAux::Literal(l) => {
                if let LiteralAux::Bin(OCamlString::String(s)) = &l.inner {
                    for char in s.chars() {
                        match char {
                            '0' => self.instruction.push(DecodeBit::Zero),
                            '1' => self.instruction.push(DecodeBit::One),
                            _ => panic!(),
                        }
                    }
                }
            }
            PatternAux::Type(typ, pat) => {
                match &*pat.inner {
                    PatternAux::Wildcard => (),
                    _ => panic!(),
                };

                match &*typ.inner {
                    TypAux::Application(ident, typargs) => {
                        if ident.inner
                            != IdentifierAux::Identifier(OCamlString::String("bits".to_owned()))
                        {
                            panic!();
                        }

                        if let TypArgAux::NExp(NumericExpression { inner: n, .. }) =
                            &typargs.front().unwrap().inner
                        {
                            if let NumericExpressionAux::Constant(sail::num::BigInt(big)) = &**n {
                                for _ in 0..big.to_u64_digits().1[0] {
                                    self.instruction.push(DecodeBit::Unknown);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }

            _ => (),
        }
        node.walk(self);
    }
}

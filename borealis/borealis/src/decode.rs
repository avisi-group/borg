//! Instruction decoding

use {
    log::{trace, warn},
    sail::{
        ast::{
            FunctionClause, IdentifierAux, Literal, LiteralAux, NumericExpression,
            NumericExpressionAux, Pattern, PatternAux, PatternMatchAux, TypArgAux, TypAux,
        },
        types::OCamlString,
        visitor::Visitor,
    },
    std::fmt::{Debug, Display},
};

/// Bit in an instruction function clause
pub enum DecodeBit {
    /// Fixed zero
    Zero,
    /// Fixed one
    One,
    /// Unknown bit
    Unknown,
}

impl Debug for DecodeBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::Unknown => write!(f, "x"),
        }
    }
}

/// Ordered collection of decode bits
#[derive(Debug)]
pub struct DecodeBits(Vec<DecodeBit>);

impl DecodeBits {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Appends a decode bit to the collection
    pub fn push(&mut self, bit: DecodeBit) {
        self.0.push(bit)
    }

    /// Moves all bits in `other` into `self`
    pub fn append(&mut self, other: &mut DecodeBits) {
        self.0.append(&mut other.0);
    }
}

impl Display for DecodeBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in &self.0 {
            write!(f, "{bit:?}")?;
        }
        Ok(())
    }
}

/// Visitor for building instruction decode strings
pub struct DecodeStringVisitor {}

impl DecodeStringVisitor {
    /// Create a new empty instance
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for DecodeStringVisitor {
    fn visit_function_clause(&mut self, node: &FunctionClause) {
        if node.inner.identifier.inner
            == IdentifierAux::Identifier(OCamlString::String("decode64".to_owned()))
        {
            process_decode_function_clause(node);
        }
    }
}

fn process_decode_function_clause(funcl: &FunctionClause) {
    trace!("Processing decode function clause @ {}", funcl.annotation.0);

    let (pat, _body) = match &funcl.inner.pattern_match.inner {
        PatternMatchAux::Expression(pat, body) => (pat, body),
        PatternMatchAux::When(pat, _, body) => {
            warn!("Function clause has condition, ignoring...");
            (pat, body)
        }
    };

    let decode_bits = extract_decode_bits(&pat.inner);

    trace!("got decode bits: {}", decode_bits);
}

fn extract_decode_bits(pattern_aux: &PatternAux) -> DecodeBits {
    let mut decode_bits = DecodeBits::new();

    let PatternAux::As(Pattern { inner, .. }, ident) = pattern_aux else {
        panic!();
    };

    assert_eq!(
        ident.to_string(),
        "Identifier(String(\"op_code\"))".to_owned()
    );

    let patterns = match &**inner {
        PatternAux::Literal(literal) => {
            trace!("op_code was a single literal!");
            return literal_to_decode_bits(literal);
        }
        PatternAux::VectorConcat(patterns) => patterns,
        v => panic!(
            "expected VectorConcat or Literal, got {}",
            Into::<&'static str>::into(v)
        ),
    };

    for pattern in patterns {
        match &*pattern.inner {
            PatternAux::Literal(literal) => {
                decode_bits.append(&mut literal_to_decode_bits(literal));
            }
            PatternAux::Type(typ, pat) => {
                match &*pat.inner {
                    PatternAux::Wildcard => (),
                    n => warn!("found non-wildcard: {}", Into::<&'static str>::into(n)),
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
                                    decode_bits.push(DecodeBit::Unknown);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            pat => warn!("pattern aux was type {}", Into::<&'static str>::into(pat)),
        }
    }

    decode_bits
}

fn literal_to_decode_bits(literal: &Literal) -> DecodeBits {
    let LiteralAux::Bin(OCamlString::String(s)) = &literal.inner else {
        panic!();
    };

    let mut decode_bits = DecodeBits::new();
    for char in s.chars() {
        match char {
            '0' => decode_bits.push(DecodeBit::Zero),
            '1' => decode_bits.push(DecodeBit::One),
            _ => panic!(),
        }
    }
    decode_bits
}

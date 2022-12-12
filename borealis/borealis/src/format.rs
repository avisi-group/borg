//! Instruction format string extraction

use {
    common::intern::InternedStringKey,
    log::{trace, warn},
    sail::{
        ast::{
            Expression, ExpressionAux, FunctionClause, Identifier, IdentifierAux, LValueExpression,
            LValueExpressionAux, Literal, LiteralAux, NumericExpression, NumericExpressionAux,
            Pattern, PatternAux, PatternMatchAux, TypArgAux, TypAux,
        },
        visitor::Visitor,
    },
    std::{
        collections::HashMap,
        fmt::{Debug, Display},
        ops::Range,
    },
};

/// Bit in an instruction format
pub enum FormatBit {
    /// Fixed zero
    Zero,
    /// Fixed one
    One,
    /// Unknown bit
    Unknown,
}

impl Debug for FormatBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::Unknown => write!(f, "x"),
        }
    }
}

/// Sequence of bits corresponding to the machine code representation of an instruction
#[derive(Debug)]
pub struct Format(Vec<FormatBit>);

impl Format {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Appends a decode bit to the collection
    pub fn push(&mut self, bit: FormatBit) {
        self.0.push(bit)
    }

    /// Moves all bits in `other` into `self`
    pub fn append(&mut self, other: &mut Format) {
        self.0.append(&mut other.0);
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in &self.0 {
            write!(f, "{bit:?}")?;
        }
        Ok(())
    }
}

/// Visitor for building instruction decode strings
pub struct DecodeStringVisitor {
    _formats: HashMap<InternedStringKey, Format>,
}

impl DecodeStringVisitor {
    /// Create a new empty instance
    pub fn new() -> Self {
        Self {
            _formats: HashMap::new(),
        }
    }
}

impl Visitor for DecodeStringVisitor {
    fn visit_function_clause(&mut self, node: &FunctionClause) {
        let IdentifierAux::Identifier(ident) = node.inner.identifier.inner else {
            return;
        };

        if ident.to_string() == "decode64" {
            process_decode_function_clause(node);
        }
    }
}

fn process_decode_function_clause(funcl: &FunctionClause) {
    trace!("Processing decode function clause @ {}", funcl.annotation.0);

    let (pat, body) = match &funcl.inner.pattern_match.inner {
        PatternMatchAux::Expression(pat, body) => (pat, body),
        PatternMatchAux::When(pat, _, body) => {
            warn!("Function clause has condition, ignoring...");
            (pat, body)
        }
    };

    let decode_bits = extract_decode_bits(&pat.inner);
    trace!("got decode bits: {}", decode_bits);

    let ExpressionAux::Block(expressions) = &*body.inner else {
        panic!("Body was not Block");
    };

    let named_ranges = expressions
        .iter()
        .map(expression_to_named_range)
        .collect::<Vec<_>>();

    dbg!(named_ranges);

    panic!();
}

fn extract_decode_bits(pattern_aux: &PatternAux) -> Format {
    let mut decode_bits = Format::new();

    let PatternAux::As(Pattern { inner, .. }, ident) = pattern_aux else {
        panic!();
    };

    assert_eq!(ident.to_string(), "Identifier(\"op_code\")".to_owned());

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
                    TypAux::Application(
                        Identifier {
                            inner: IdentifierAux::Identifier(s),
                            ..
                        },
                        typargs,
                    ) => {
                        if s.to_string() != "bits" {
                            panic!();
                        }

                        if let TypArgAux::NExp(NumericExpression { inner: n, .. }) =
                            &typargs.front().unwrap().inner
                        {
                            if let NumericExpressionAux::Constant(sail::num::BigInt(big)) = &**n {
                                for _ in 0..big.to_u64_digits().1[0] {
                                    decode_bits.push(FormatBit::Unknown);
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

fn literal_to_decode_bits(literal: &Literal) -> Format {
    let LiteralAux::Bin(s) = &literal.inner else {
        panic!("Unexpected literal when decoding instruction format");
    };

    let mut decode_bits = Format::new();
    for char in s.to_string().chars() {
        match char {
            '0' => decode_bits.push(FormatBit::Zero),
            '1' => decode_bits.push(FormatBit::One),
            c => panic!("Unexpected char {:?} when decoding instruction format", c),
        }
    }
    decode_bits
}

fn expression_to_named_range(expression: &Expression) -> (InternedStringKey, Range<usize>) {
    trace!("is SEE assignment = {:?}", is_see_assignment(expression));

    ("?".into(), 0..1)
}

/// Tests whether an expression is an assignment to `SEE`
fn is_see_assignment(expression: &Expression) -> bool {
    let ExpressionAux::Assign(LValueExpression { inner, ..}, _) = &*expression.inner else {
        return false;
    };

    let LValueExpressionAux::Identifier(Identifier {inner: IdentifierAux::Identifier(s),..}) = &**inner else {
        return false;
    };

    s.to_string() == "SEE"
}

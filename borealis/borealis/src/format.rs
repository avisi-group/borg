//! Instruction format string extraction

use {
    common::intern::InternedStringKey,
    log::{trace, warn},
    num_bigint::Sign,
    sail::{
        ast::{
            Expression, ExpressionAux, FunctionClause, Identifier, IdentifierAux, LValueExpression,
            LValueExpressionAux, Literal, LiteralAux, NumericExpression, NumericExpressionAux,
            Pattern, PatternAux, PatternMatchAux, TypArgAux, TypAux,
        },
        num::BigInt,
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

impl From<&Literal> for Format {
    fn from(literal: &Literal) -> Self {
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

    let decode_bits = extract_format(&pat.inner);
    trace!("got decode bits: {}", decode_bits);

    let ExpressionAux::Block(expressions) = &*body.inner else {
        panic!("Body was not Block");
    };

    let expression_count = expressions.len();

    assert!(is_see_assignment(expressions.front().unwrap()));
    //   assert!(is_instruction_impl_function_call(expressions.back().unwrap()));

    let named_ranges = expressions
        .iter()
        .take(expression_count - 1) // skip last expression
        .skip(1) // skip first expression
        .filter_map(expression_to_named_range)
        .collect::<Vec<_>>();

    trace!("ranges: {:?}", named_ranges);
}

fn extract_format(pattern_aux: &PatternAux) -> Format {
    let mut decode_bits = Format::new();

    let PatternAux::As(Pattern { inner, .. }, ident) = pattern_aux else {
        panic!();
    };

    assert_eq!(ident.to_string(), "Identifier(\"op_code\")".to_owned());

    let patterns = match &**inner {
        PatternAux::Literal(literal) => {
            trace!("op_code was a single literal!");
            return Format::from(literal);
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
                decode_bits.append(&mut Format::from(literal));
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

fn expression_to_named_range(expression: &Expression) -> Option<(InternedStringKey, Range<usize>)> {
    let ExpressionAux::Assign(LValueExpression { inner: lvalue, ..}, Expression { inner: expression_aux, .. }) = &*expression.inner else {
        panic!("ExpressionAux not an Assign");

    };

    let name = {
        let LValueExpressionAux::Cast(_, ident) = &**lvalue else {
            panic!("LValueExpression not a Cast");
        };

        ident.get_string()
    };

    let range = match &**expression_aux {
        ExpressionAux::Vector(expressions) => {
            assert_eq!(expressions.len(), 1);
            bitvector_access_to_range(&*expressions.front().unwrap().inner)
        }
        ExpressionAux::Application(_, _) => vector_subrange_to_range(expression_aux),
        exp => panic!(
            "Unexpected Expression variant {:?}",
            <&'static str>::from(exp)
        ),
    };

    Some((name, range))
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

fn bitvector_access_to_range(exp: &ExpressionAux) -> Range<usize> {
    let ExpressionAux::Application(ident, expressions) = exp else {
        panic!("not an application");
    };
    assert_eq!(ident.get_string(), "bitvector_access_A".into());

    let mut iter = expressions.iter();

    // first expression should be the `op_code` identifier
    let ExpressionAux::Identifier(ident) = &*iter.next().unwrap().inner else {
        panic!("first expression was not identifier");
    };
    assert_eq!(ident.get_string(), "op_code".into());

    let start = expression_to_usize(iter.next().unwrap());
    let end = start + 1;

    assert!(iter.next().is_none());

    start..end
}

fn vector_subrange_to_range(exp: &ExpressionAux) -> Range<usize> {
    let ExpressionAux::Application(ident, expressions) = exp else {
        panic!("not an application");
    };
    assert_eq!(ident.get_string(), "vector_subrange_A".into());

    let mut iter = expressions.iter();

    // first expression should be the `op_code` identifier
    let ExpressionAux::Identifier(ident) = &*iter.next().unwrap().inner else {
        panic!("first expression was not identifier");
    };
    assert_eq!(ident.get_string(), "op_code".into());

    let start = expression_to_usize(iter.next().unwrap());
    let end = expression_to_usize(iter.next().unwrap());

    assert!(iter.next().is_none());

    start..end
}

fn expression_to_usize(expression: &Expression) -> usize {
    let ExpressionAux::Literal(Literal { inner: LiteralAux::Num(BigInt(bigint)), .. }) = &*expression.inner else {
        panic!("expression was not a num literal");
    };

    let (sign, digits) = bigint.to_u64_digits();

    assert!(sign != Sign::Minus);

    match digits.len() {
        0 => 0,
        1 => usize::try_from(digits[0]).unwrap(),
        _ => panic!("bigint {:?} too large", bigint),
    }
}

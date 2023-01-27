//! Instruction format string extraction

use {
    crate::genc::format::{InstructionFormat as GenCFormat, Segment, SegmentContent},
    common::{identifiable::unique_id, intern::InternedStringKey},
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
        collections::{HashMap, LinkedList},
        fmt::{Debug, Display},
        ops::Range,
    },
};

/// Bit in an instruction format
#[derive(PartialEq, Eq)]
pub enum FormatBit {
    /// Fixed zero
    Zero,
    /// Fixed one
    One,
    /// Unknown bit
    Unknown,
}

impl FormatBit {
    fn is_unknown(&self) -> bool {
        match self {
            Self::Zero | Self::One => false,
            Self::Unknown => true,
        }
    }

    fn is_fixed(&self) -> bool {
        !self.is_unknown()
    }
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
    pub fn empty() -> Self {
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

    /// Finish building a sequence of format bits
    pub fn finish(mut self) -> Vec<FormatBit> {
        self.0.reverse();
        self.0
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

        let mut decode_bits = Format::empty();
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
    /// Decoded instruction formats
    pub formats: HashMap<InternedStringKey, GenCFormat>,
    counters: HashMap<InternedStringKey, u32>,
}

impl DecodeStringVisitor {
    /// Create a new empty instance
    pub fn new() -> Self {
        Self {
            formats: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    fn increment_count(&mut self, key: InternedStringKey) -> u32 {
        let current = self.counters.get(&key).copied().unwrap_or(0);
        let last = self.counters.insert(key, current + 1);
        assert!(last == Some(current) || last.is_none());
        current
    }
}

impl Default for DecodeStringVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for DecodeStringVisitor {
    fn visit_function_clause(&mut self, node: &FunctionClause) {
        let IdentifierAux::Identifier(ident) = node.inner.identifier.inner else {
            return;
        };

        if ident.to_string() == "decode64" {
            process_decode_function_clause(self, node);
        }
    }
}

/// Main function clause processing
fn process_decode_function_clause(visitor: &mut DecodeStringVisitor, funcl: &FunctionClause) {
    trace!(
        "Processing decode function clause @ {}",
        funcl.annotation.location
    );

    let (pat, body) = match &funcl.inner.pattern_match.inner {
        PatternMatchAux::Expression(pat, body) => (pat, body),
        PatternMatchAux::When(pat, _, body) => {
            warn!("Function clause has condition, ignoring...");
            (pat, body)
        }
    };

    let format_bits = extract_format(&pat.inner);

    let ExpressionAux::Block(expressions) = &*body.inner else {
        panic!("Body was not Block");
    };

    assert_eq!(expressions.len(), 2);

    assert!(is_see_assignment(expressions.front().unwrap()));

    let expressions = flatten_expression(expressions.back().unwrap());

    let mut named_ranges = expressions
        .iter()
        .take(expressions.len() - 1) // skip last expression
        .filter_map(expression_to_named_range)
        .collect::<Vec<_>>();

    let instruction_name = {
        let Some(Expression { inner, .. }) = expressions.last() else { panic!() };

        let ExpressionAux::Application(ident, ..) = &**inner else { panic!() };

        ident.get_string()
    };

    // named ranges may not be contiguous, so fill any holes with padding ranges (these should not contain any unknown bits)
    {
        let mut padding_ranges = vec![];
        let mut last_end = 0;
        for (_, range) in &named_ranges {
            if range.start != last_end {
                // insert new range from last_end to range.start
                let padding = (
                    format!("padding{}", unique_id()).into(),
                    last_end..range.start,
                );
                warn!("inserting new padding range: {:?}", padding);
                padding_ranges.push(padding);
            }
            last_end = range.end;
        }

        if last_end != format_bits.len() {
            let padding = (
                format!("padding{}", unique_id()).into(),
                last_end..format_bits.len(),
            );
            warn!("inserting final padding range: {:?}", padding);
            padding_ranges.push(padding);
        }

        named_ranges.append(&mut padding_ranges);
        named_ranges.sort_by(|(_, a), (_, b)| a.start.cmp(&b.start));
    }

    trace!("named_ranges: {:?}", named_ranges);

    let homogenised_ranges = named_ranges
        .clone()
        .into_iter()
        .flat_map(|(n, r)| {
            let bits = &format_bits[r.clone()];

            // determine locations where bit kind changes
            let delta = bits
                .windows(2)
                .map(|window| window[0].is_unknown() != window[1].is_unknown())
                .collect::<Vec<_>>();

            // indexes where ranges must be split
            let split_indexes = delta
                .iter()
                .enumerate()
                .filter_map(|(i, x)| match x {
                    true => Some(i + 1),
                    false => None,
                })
                .collect::<Vec<_>>();

            let mut new_ranges = vec![];
            let mut current_start = r.start;
            for split_index in split_indexes {
                new_ranges.push(current_start..r.start + split_index);
                current_start = r.start + split_index;
            }
            new_ranges.push(current_start..r.end);

            new_ranges.into_iter().enumerate().map(move |(i, range)| {
                if i == 0 {
                    (n, range)
                } else {
                    (format!("{}_part{}", &n, i).into(), range)
                }
            })
        })
        .collect::<Vec<_>>();

    trace!("homogenised_ranges: {:?}", homogenised_ranges);

    let homogenised_bits = homogenised_ranges
        .into_iter()
        .map(|(n, r)| (n, &format_bits[r]))
        .collect::<Vec<_>>();

    trace!("homogenised_bits: {:?}", homogenised_bits);

    let mut inner = homogenised_bits
        .into_iter()
        .map(|(n, bits)| {
            let content = if bits.iter().all(FormatBit::is_unknown) {
                SegmentContent::Variable(n)
            } else if bits.iter().all(FormatBit::is_fixed) {
                let mut binary_string = "".to_owned();
                for bit in bits {
                    match bit {
                        FormatBit::Zero => binary_string.push('0'),
                        FormatBit::One => binary_string.push('1'),
                        FormatBit::Unknown => panic!(),
                    }
                }

                SegmentContent::Constant(u64::from_str_radix(&binary_string, 2).unwrap())
            } else {
                panic!();
            };

            Segment {
                content,
                length: bits.len(),
            }
        })
        .collect::<Vec<_>>();

    inner.reverse();

    assert_eq!(inner.iter().map(|s| s.length).sum::<usize>(), 32);

    let count = visitor.increment_count(instruction_name);

    let name = format!("{}{}", instruction_name, count).into();
    let format = GenCFormat(inner);
    trace!("{} genc format: {}", name, format);

    visitor.formats.insert(name, format);
}

fn flatten_expression(expression: &Expression) -> Vec<Expression> {
    let mut exps = vec![];

    let mut current_expression = expression.clone();

    loop {
        let (lhs, rhs, exp1) = match *current_expression.inner {
            ExpressionAux::Var(lhs, rhs, exp1) => (lhs, rhs, exp1),
            ExpressionAux::Application(..) => {
                exps.push(current_expression);
                break;
            }
            _ => panic!("{}", <&'static str>::from(*current_expression.inner)),
        };

        current_expression.inner = Box::new(ExpressionAux::Assign(lhs.clone(), rhs.clone()));

        exps.push(current_expression);

        let ExpressionAux::Block(expressions) = &*exp1.inner else { panic!() };

        match expressions.len() {
            0 => break,
            1 => {
                current_expression = expressions.front().unwrap().clone();
            }
            _ => panic!(),
        }
    }

    exps
}

fn extract_format(pattern_aux: &PatternAux) -> Vec<FormatBit> {
    let mut format_bits = Format::empty();

    let PatternAux::As(Pattern { inner, .. }, ident) = pattern_aux else {
        panic!();
    };

    assert_eq!(ident.to_string(), "Identifier(\"op_code\")".to_owned());

    let patterns = match &**inner {
        PatternAux::Literal(literal) => {
            trace!("op_code was a single literal!");
            return Format::from(literal).finish();
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
                format_bits.append(&mut Format::from(literal));
            }
            PatternAux::Type(typ, pat) => {
                match &*pat.inner {
                    PatternAux::Wildcard => (),
                    n => warn!("found non-wildcard: {}", Into::<&'static str>::into(n)),
                };

                if let TypAux::Application(
                    Identifier {
                        inner: IdentifierAux::Identifier(s),
                        ..
                    },
                    typargs,
                ) = &*typ.inner
                {
                    if s.to_string() != "bits" {
                        panic!();
                    }

                    if let TypArgAux::NExp(NumericExpression { inner: n, .. }) =
                        &typargs.front().unwrap().inner
                    {
                        if let NumericExpressionAux::Constant(sail::num::BigInt(big)) = &**n {
                            for _ in 0..big.to_u64_digits().1[0] {
                                format_bits.push(FormatBit::Unknown);
                            }
                        }
                    }
                }
            }
            pat => warn!("pattern aux was type {}", Into::<&'static str>::into(pat)),
        }
    }

    trace!("got format bits: {}", format_bits);

    format_bits.finish()
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
            bitvector_access_to_range(&expressions.front().unwrap().inner)
        }
        ExpressionAux::Application(ident, exps) => vector_subrange_to_range(ident, exps),
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

/// Extracts a range from a `bitvector_access` function application
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

/// Extracts a range from a `vector_subrange` function application
fn vector_subrange_to_range(
    ident: &Identifier,
    expressions: &LinkedList<Expression>,
) -> Range<usize> {
    assert_eq!(ident.get_string(), "vector_subrange_A".into());

    let mut iter = expressions.iter();

    // first expression should be the `op_code` identifier
    let ExpressionAux::Identifier(ident) = &*iter.next().unwrap().inner else {
        panic!("first expression was not identifier");
    };
    assert_eq!(ident.get_string(), "op_code".into());

    let end = expression_to_usize(iter.next().unwrap()) + 1;
    let start = expression_to_usize(iter.next().unwrap());

    assert!(iter.next().is_none());

    start..end
}

/// Extracts a `usize` from a Literal expression
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

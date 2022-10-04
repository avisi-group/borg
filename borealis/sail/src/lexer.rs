use {
    crate::{
        error::Error,
        span::{Span, Spanned},
    },
    chumsky::{
        error::Simple,
        primitive::{choice, end, filter, just, one_of, take_until},
        text::{self, TextParser},
        Parser,
    },
    std::{fmt::Display, ops::Range},
    strum::{EnumCount, EnumIter, IntoEnumIterator},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Keyword {
    And,
    As,
    Assert,
    Bitzero,
    Bitone,
    By,
    Match,
    Clause,
    Dec,
    Op,
    Default,
    Effect,
    End,
    Enum,
    Else,
    Exit,
    Cast,
    False,
    Forall,
    Foreach,
    Function,
    Mapping,
    Overload,
    Throw,
    Try,
    Catch,
    If,
    In,
    Inc,
    Let,
    Var,
    Ref,
    Int,
    Order,
    Bool,
    Pure,
    Monadic,
    Register,
    Return,
    Scattered,
    Sizeof,
    Constraint,
    Constant,
    Struct,
    Then,
    True,
    Type,
    Typedef,
    Undefined,
    Union,
    Newtype,
    With,
    Val,
    Outcome,
    Instantiation,
    Impl,
    Repeat,
    Until,
    While,
    Do,
    Mutual,
    Bitfield,
    Barr,
    Depend,
    Rreg,
    Wreg,
    Rmem,
    Wmem,
    Wmv,
    Eamem,
    Exmem,
    Undef,
    Unspec,
    Nondet,
    Escape,
    Configuration,
    TerminationMeasure,
    Forwards,
    Backwards,
    InternalPLet,
    InternalReturn,
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "and" => Ok(Keyword::And),
            "as" => Ok(Keyword::As),
            "assert" => Ok(Keyword::Assert),
            "bitzero" => Ok(Keyword::Bitzero),
            "bitone" => Ok(Keyword::Bitone),
            "by" => Ok(Keyword::By),
            "match" => Ok(Keyword::Match),
            "clause" => Ok(Keyword::Clause),
            "dec" => Ok(Keyword::Dec),
            "operator" => Ok(Keyword::Op),
            "default" => Ok(Keyword::Default),
            "effect" => Ok(Keyword::Effect),
            "end" => Ok(Keyword::End),
            "enum" => Ok(Keyword::Enum),
            "else" => Ok(Keyword::Else),
            "exit" => Ok(Keyword::Exit),
            "cast" => Ok(Keyword::Cast),
            "false" => Ok(Keyword::False),
            "forall" => Ok(Keyword::Forall),
            "foreach" => Ok(Keyword::Foreach),
            "function" => Ok(Keyword::Function),
            "mapping" => Ok(Keyword::Mapping),
            "overload" => Ok(Keyword::Overload),
            "throw" => Ok(Keyword::Throw),
            "try" => Ok(Keyword::Try),
            "catch" => Ok(Keyword::Catch),
            "if" => Ok(Keyword::If),
            "in" => Ok(Keyword::In),
            "inc" => Ok(Keyword::Inc),
            "let" => Ok(Keyword::Let),
            "var" => Ok(Keyword::Var),
            "ref" => Ok(Keyword::Ref),
            "Int" => Ok(Keyword::Int),
            "Order" => Ok(Keyword::Order),
            "Bool" => Ok(Keyword::Bool),
            "pure" => Ok(Keyword::Pure),
            "monadic" => Ok(Keyword::Monadic),
            "register" => Ok(Keyword::Register),
            "return" => Ok(Keyword::Return),
            "scattered" => Ok(Keyword::Scattered),
            "sizeof" => Ok(Keyword::Sizeof),
            "constraint" => Ok(Keyword::Constraint),
            "constant" => Ok(Keyword::Constant),
            "struct" => Ok(Keyword::Struct),
            "then" => Ok(Keyword::Then),
            "true" => Ok(Keyword::True),
            "Type" => Ok(Keyword::Type),
            "type" => Ok(Keyword::Typedef),
            "undefined" => Ok(Keyword::Undefined),
            "union" => Ok(Keyword::Union),
            "newtype" => Ok(Keyword::Newtype),
            "with" => Ok(Keyword::With),
            "val" => Ok(Keyword::Val),
            "outcome" => Ok(Keyword::Outcome),
            "instantiation" => Ok(Keyword::Instantiation),
            "impl" => Ok(Keyword::Impl),
            "repeat" => Ok(Keyword::Repeat),
            "until" => Ok(Keyword::Until),
            "while" => Ok(Keyword::While),
            "do" => Ok(Keyword::Do),
            "mutual" => Ok(Keyword::Mutual),
            "bitfield" => Ok(Keyword::Bitfield),
            "barr" => Ok(Keyword::Barr),
            "depend" => Ok(Keyword::Depend),
            "rreg" => Ok(Keyword::Rreg),
            "wreg" => Ok(Keyword::Wreg),
            "rmem" => Ok(Keyword::Rmem),
            "rmemt" => Ok(Keyword::Rmem),
            "wmem" => Ok(Keyword::Wmem),
            "wmv" => Ok(Keyword::Wmv),
            "wmvt" => Ok(Keyword::Wmv),
            "eamem" => Ok(Keyword::Eamem),
            "exmem" => Ok(Keyword::Exmem),
            "undef" => Ok(Keyword::Undef),
            "unspec" => Ok(Keyword::Unspec),
            "nondet" => Ok(Keyword::Nondet),
            "escape" => Ok(Keyword::Escape),
            "configuration" => Ok(Keyword::Configuration),
            "termination_measure" => Ok(Keyword::TerminationMeasure),
            "forwards" => Ok(Keyword::Forwards),
            "backwards" => Ok(Keyword::Backwards),
            "internal_plet" => Ok(Keyword::InternalPLet),
            "internal_return" => Ok(Keyword::InternalReturn),
            "import" => Err("\"import\" is a reserved keyword"),
            "module" => Err("\"module\" is a reserved keyword"),
            _ => Err("unknown keyword"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter, EnumCount)]
pub enum Operator {
    Bang,
    Percent,
    Ampersand,
    Asterisk,
    Plus,
    Minus,
    Period,
    ForwardSlash,
    Colon,
    LessThan,
    Equals,
    GreaterThan,
    At,
    Caret,
    Pipe,
}

impl From<Operator> for char {
    fn from(o: Operator) -> Self {
        match o {
            Operator::Bang => '!',
            Operator::Percent => '%',
            Operator::Ampersand => '&',
            Operator::Asterisk => '*',
            Operator::Plus => '+',
            Operator::Minus => '-',
            Operator::Period => '.',
            Operator::ForwardSlash => '/',
            Operator::Colon => ':',
            Operator::LessThan => '<',
            Operator::Equals => '=',
            Operator::GreaterThan => '>',
            Operator::At => '@',
            Operator::Caret => '^',
            Operator::Pipe => '|',
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '!' => Ok(Operator::Bang),
            '%' => Ok(Operator::Percent),
            '&' => Ok(Operator::Ampersand),
            '*' => Ok(Operator::Asterisk),
            '+' => Ok(Operator::Plus),
            '-' => Ok(Operator::Minus),
            '.' => Ok(Operator::Period),
            '/' => Ok(Operator::ForwardSlash),
            ':' => Ok(Operator::Colon),
            '<' => Ok(Operator::LessThan),
            '=' => Ok(Operator::Equals),
            '>' => Ok(Operator::GreaterThan),
            '@' => Ok(Operator::At),
            '^' => Ok(Operator::Caret),
            '|' => Ok(Operator::Pipe),
            _ => Err("unknown operator"),
        }
    }
}

impl Operator {
    pub fn all() -> [char; Self::COUNT] {
        let mut all = ['\0'; Self::COUNT];

        for (i, s) in Self::iter().enumerate() {
            all[i] = s.into();
        }

        all
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Ampersand,
    At,
    TwoCaret,
    Caret,
    ColonColon,
    TildeTilde,
    Colon,
    Comma,
    DotDot,
    Dot,
    EqualsEquals,
    Equals,
    GreaterThan,
    Minus,
    LessThan,
    Plus,
    Semicolon,
    Star,
    Underscore,
    LeftSquareBar,
    RightSquareBar,
    LeftCurlyBar,
    RightCurlyBar,
    Bar,
    LeftCurly,
    RightCurly,
    Unit,
    LeftParenthesis,
    RightParenthesis,
    LeftSquare,
    RightSquare,
    ExclamationEquals,
    GreaterThanEquals,
    MinusGreaterThan,
    Bidirectional,
    EqualsGreaterThan,
    LessThanEquals,
    Pragma(String),
    Infix((u32, Operator)),
    InfixLeft((u32, Operator)),
    InfixRight((u32, Operator)),
    Operator(Operator),
    TyVar(String),
    Identifier(String), // including "~"
    Keyword(Keyword),
    Real(String),
    Num(String),
    Bin(String),
    Hex(String),
    String(String),
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Error<char>> {
    let simple_tokens = choice((
        just('&').to(Token::Ampersand),
        just('@').to(Token::At),
        just('2').padded().chain(just('^')).to(Token::TwoCaret),
        just('^').to(Token::Caret),
        just("::").to(Token::ColonColon),
        just("~~").to(Token::TildeTilde),
        just(":").to(Token::Colon),
        just(",").to(Token::Comma),
        just("..").to(Token::DotDot),
        just(".").to(Token::Dot),
        just("==").to(Token::EqualsEquals),
        just("=").to(Token::Equals),
        just(">").to(Token::GreaterThan),
        just("-").to(Token::Minus),
        just("<").to(Token::LessThan),
        just("+").to(Token::Plus),
        just(";").to(Token::Semicolon),
        just("*").to(Token::Star),
        just("[|").to(Token::LeftSquareBar),
        just("|]").to(Token::RightSquareBar),
        just("{|").to(Token::LeftCurlyBar),
        just("|}").to(Token::RightCurlyBar),
        just("|").to(Token::Bar),
        just("{").to(Token::LeftCurly),
    ))
    // split due to limits on tuple size
    .or(choice((
        just("}").to(Token::RightCurly),
        just("()").to(Token::Unit),
        just("(").to(Token::LeftParenthesis),
        just(")").to(Token::RightParenthesis),
        just("[").to(Token::LeftSquare),
        just("]").to(Token::RightSquare),
        just("!=").to(Token::ExclamationEquals),
        just(">=").to(Token::GreaterThanEquals),
        just("->").to(Token::MinusGreaterThan),
        just("<->").to(Token::Bidirectional),
        just("=>").to(Token::EqualsGreaterThan),
        just("<=").to(Token::LessThanEquals),
    )));

    let pragma = just('$')
        .ignore_then(filter(|c| *c != '\n').repeated())
        .then_ignore(just('\n'))
        .collect::<String>()
        .map(Token::Pragma);

    let operator = one_of(Operator::all()).map(|s: char| Operator::try_from(s).unwrap());

    let digit = filter(|c: &char| c.is_digit(10)).map(|c| {
        c.to_digit(10)
            .expect("is_digit is true means to_digit must always succeed")
    });

    let (infix, infix_left, infix_right) = {
        let infix = just("infix")
            .padded()
            .ignore_then(digit)
            .padded()
            .then(operator.clone())
            .map(Token::Infix);

        let infix_left = just("infixl")
            .padded()
            .ignore_then(digit)
            .padded()
            .then(operator.clone())
            .map(Token::InfixLeft);

        let infix_right = just("infixr")
            .padded()
            .ignore_then(digit)
            .padded()
            .then(operator.clone())
            .map(Token::InfixRight);

        (infix, infix_left, infix_right)
    };

    let operator_token = operator.map(Token::Operator);

    let identifier = filter(|&c: &char| c.is_ascii_alphabetic() || c == '?' || c == '_')
        .map(Some)
        .chain::<char, _, _>(
            filter(|&c: &char| {
                c.is_ascii_alphanumeric() || c == '?' || c == '_' || c == '\'' || c == '#'
            })
            .repeated(),
        )
        .collect::<String>();

    let ty_var = just('\'')
        .chain(identifier)
        .collect::<String>()
        .map(Token::TyVar);

    let tilde = just('~').to(Token::Identifier("~".to_owned()));

    let identifier_token = identifier.map(|s| match Keyword::try_from(s.as_str()) {
        Ok(kw) => Token::Keyword(kw),
        Err(_) => Token::Identifier(s),
    });

    let num = just('-')
        .or_not()
        .chain::<char, _, _>(text::digits(10))
        .collect::<String>()
        .map(Token::Num);

    let real = just('-')
        .or_not()
        .chain::<char, _, _>(text::digits(10))
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Token::Real);

    let bin = just("0b")
        .ignore_then(
            filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
                .repeated()
                .at_least(1),
        )
        .validate(|chars, span: Range<usize>, emit| {
            chars.iter().for_each(|c| {
                if !(c.is_digit(2) || *c == '_') {
                    emit(Error(Simple::custom(
                        span.clone(),
                        format!(
                            "Encountered non-binary character \"{c}\" while parsing binary literal"
                        ),
                    )))
                }
            });

            chars
        })
        .collect::<String>()
        .map(Token::Bin);

    let hex = just("0x")
        .ignore_then(
            filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
                .repeated()
                .at_least(1),
        )
        .validate(|chars, span: Range<usize>, emit| {
            chars.iter().for_each(|c| {
                if !(c.is_digit(16) || *c == '_') {
                    emit(Error(Simple::custom(
                        span.clone(),
                        format!("Encountered non-hexadecimal character \"{c}\" while parsing hexadecimal literal"),
                    )))
                }
            });

            chars
        })
        .collect::<String>()
        .map(Token::Hex);

    let string = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::String);

    let token = choice((
        simple_tokens,
        pragma,
        infix,
        infix_left,
        infix_right,
        operator_token,
        ty_var,
        tilde,
        identifier_token,
        just("_").to(Token::Underscore),
        hex,
        bin,
        num,
        real,
        string,
    ));

    let line_comment = just("//").then(take_until(just('\n'))).padded();
    let comment = just("/*").then(take_until(just("*/"))).padded();

    token
        .map_with_span(|tok, span| (tok, Span::from(span)))
        .padded_by(line_comment.repeated())
        .padded_by(comment.repeated())
        .padded()
        .repeated()
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use {
        crate::lexer::{lexer, Operator},
        chumsky::Parser,
        insta::assert_debug_snapshot,
        proptest::prelude::*,
        strum::IntoEnumIterator,
    };

    #[test]
    fn operator_char_parsing() {
        for op in Operator::iter() {
            assert_eq!(op, char::from(op).try_into().unwrap());
        }
    }

    #[test]
    fn aarch64_prelude_snapshot() {
        assert_debug_snapshot!(lexer()
            .parse(include_str!("../examples/prelude.sail"))
            .unwrap())
    }

    proptest! {
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            lexer().parse(s).ok();
        }
    }
}

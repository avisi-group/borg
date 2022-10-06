//! Main parser implementation and AST definition

use {
    crate::{
        ast::{
            Atyp, Definition, EffectAnnotation, FunctionDefinition, Identifier, MappingDefinition,
            Pattern, RecursiveAnnotation, TypeAnnotation,
        },
        error::Error,
        lexer::{Keyword, Token},
        span::Spanned,
    },
    chumsky::{
        primitive::{choice, just},
        select, Parser,
    },
};

/// Gets new instance of Sail parser
pub fn parser() -> impl Parser<Token, Vec<Spanned<Definition>>, Error = Error<Token>> {
    //let id = select! {Token::Identifier(s) => Identifier::Identifier(s)};

    let typ = just(Token::LeftCurly).to(Atyp::Decreasing);

    let pat1 = just(Token::LeftCurly).to(Pattern::Wildcard);

    // ⟨pat⟩ ::= ⟨pat1⟩
    //        |  ⟨pat1⟩ as ⟨typ⟩
    //        |  ⟨pat1⟩ match ⟨typ⟩
    let pat = choice((
        pat1.clone(),
        pat1.clone()
            .then_ignore(just(Token::Keyword(Keyword::As)))
            .then(typ.clone())
            .map(|(pat1, typ)| Pattern::Variable(Box::new(pat1), typ)),
        pat1.clone()
            .then_ignore(just(Token::Keyword(Keyword::Match)))
            .then(typ.clone())
            .map(|(pat1, typ)| Pattern::Variable(Box::new(pat1), typ)),
    ));

    let exp = just(Token::LeftCurly);

    // ⟨rec measure⟩ ::= { ⟨pat⟩ => ⟨exp⟩ }
    let rec_measure = just(Token::LeftCurly)
        .ignore_then(pat)
        .then_ignore(just(Token::Equals))
        .then_ignore(just(Token::GreaterThan))
        .then(exp)
        .then_ignore(just(Token::RightCurly));

    let funcls = just(Token::LeftCurly);

    // ⟨fun def⟩ ::= function [⟨rec measure⟩] ⟨funcls⟩
    let fun_def = just(Token::Keyword(Keyword::Function))
        .ignore_then(rec_measure.or_not())
        .then(funcls)
        .map(|_| {
            Definition::Function(FunctionDefinition(
                RecursiveAnnotation::NonRecursive,
                TypeAnnotation(None),
                EffectAnnotation(None),
                vec![],
            ))
        });

    let typschm = just(Token::Ampersand);

    // ⟨mapcl_list⟩ ::= ⟨mapcl_doc⟩ [,]
    //               |  ⟨mapcl_doc⟩ , ⟨mapcl_list⟩
    let mapcl_list = just(Token::Ampersand);

    // ⟨map_def⟩ ::= mapping ⟨id⟩ = { ⟨mapcl_list⟩ }
    //            |  mapping ⟨id⟩ : ⟨typschm⟩ = { ⟨mapcl_list⟩ }
    let map_def = just(Token::Keyword(Keyword::Mapping))
        .ignore_then(select! {Token::Identifier(s) => s})
        .then(just(Token::Colon).ignore_then(typschm).or_not())
        .then_ignore(just(Token::Equals))
        .then(mapcl_list.delimited_by(just(Token::LeftCurly), just(Token::RightCurly)))
        .map(|_| {
            Definition::Mapping(MappingDefinition(
                Identifier::Identifier("".to_owned()),
                None,
                vec![],
            ))
        });

    let pragma = select! { Token::Pragma(s) => Definition::Pragma(s.clone(), s)};

    let definition = choice((
        fun_def, map_def,
        // function_implementation,
        // infix_operator,
        // value_specification,
        // outcome_specification,
        // instantiation,
        // type_definition,
        // let_bind,
        // register,
        // overload,
        // scattered,
        // default,
        // mutual,
        pragma,
        // termination,
    ));

    definition.map_with_span(|def, span| (def, span)).repeated()
}

#[cfg(test)]
mod tests {
    use {
        crate::{lexer, parser},
        chumsky::{Parser, Stream},
        insta::assert_debug_snapshot,
    };

    #[test]
    fn aarch64_prelude_snapshot() {
        let source = include_str!("../examples/prelude.sail");
        let tokens = lexer().parse(source).unwrap();
        assert_debug_snapshot!(parser()
            .parse(Stream::from_iter(
                source.len()..source.len() + 1,
                tokens.into_iter()
            ))
            .unwrap())
    }
}

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ott.pest"]
pub(crate) struct Parser;

#[cfg(test)]
mod tests {
    use {
        crate::ott::{Parser, Rule},
        pest::Parser as _,
    };

    #[test]
    fn parse_leroy_jfp96() {
        insta::assert_debug_snapshot!(Parser::parse(
            Rule::main,
            include_str!("../testdata/leroy-jfp96.ott")
        )
        .unwrap());
    }

    #[test]
    fn parse_ocaml() {
        insta::assert_debug_snapshot!(Parser::parse(
            Rule::main,
            include_str!("../testdata/ocaml_syntax.ott")
        )
        .unwrap());
    }

    #[test]
    fn parse_test8() {
        insta::assert_debug_snapshot!(Parser::parse(
            Rule::main,
            include_str!("../testdata/test8.ott")
        )
        .unwrap());
    }

    #[test]
    fn parse_test10() {
        insta::assert_debug_snapshot!(Parser::parse(
            Rule::main,
            include_str!("../testdata/test10.ott")
        )
        .unwrap());
    }

    #[test]
    fn parse_test10st() {
        insta::assert_debug_snapshot!(Parser::parse(
            Rule::main,
            include_str!("../testdata/test10st.ott")
        )
        .unwrap());
    }
}

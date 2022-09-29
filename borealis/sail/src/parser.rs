// Gets new instance of Sail parser
// pub fn parser() -> impl Parser<char, Definition, Error = Error> {
//     todo!()
// }

/// Sail definition
#[derive(Debug)]
pub enum Definition {
    FunctionDefinition(FunctionDefinition),
    MapDefinition(MapDefinition),
    ImplFunction(Funcl),
    InfixOperator,
}

#[derive(Debug)]
pub enum FunctionDefinition {}

#[derive(Debug)]
pub enum MapDefinition {}

#[derive(Debug)]
pub enum Funcl {}

#[derive(Debug)]
pub enum Expression {}

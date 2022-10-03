//! Main parser implementation and AST definition

use {
    crate::{lexer::Operator, span::Span},
    num_bigint::BigInt,
};

/// Top-level Sail2 definition
#[derive(Debug, Clone)]
pub enum Definition {
    /// Type definition
    Type(TypeDefinition),

    /// Function definition
    Function(FunctionDefinition),

    /// Mapping definition
    Mapping(MappingDefinition),

    /// Implementation definition (`funcl` in Sail2 internals)
    Implementation(ImplementationDefinition),

    /// Let bind value definition
    Let(LetBind),

    /// Operator overload specifications
    Overload {
        identifier: String,
        ids: Vec<String>,
    },

    /// Fixity declaration
    InfixOperator {
        kind: InfixKind,
        digit: u8,
        operator: Operator,
    },

    /// Top-level type constraint
    ValueSpec(ValueSpec),

    /// Top-level outcome definition
    OutcomeSpec(Vec<Definition>),

    /// Instantiation
    Instantiation {
        identifier: String,
        subst: Vec<()>,
    },

    /// Default type and kind assumptions
    Default {
        kind: Kind,
        direction: Direction,
    },

    Scattered(ScatteredDefinition),

    /// Separate termination measure declaration
    TerminationMeasurePatternExpression {
        identifier: String,
        pattern: Pattern,
        expression: Expression,
    },

    /// Separate termination measure declaration
    TerminationMeasureLoop {
        identifier: String,
        loop_measure: Vec<LoopMeasure>,
    },

    /// Register declaration
    Register(RegisterDefinition),

    /// Pragma
    Pragma(String, String, L),

    /// Internal mutrec
    Mutual(Vec<FunctionDefinition>),
}

/// Type definition
#[derive(Debug, Clone)]
pub struct TypeDefinition {}

/// Function definition
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub rec_measure: RecMeasure,
    pub funcls: Funcls,
}

/// Mapping definition
#[derive(Debug, Clone)]
pub struct MappingDefinition {}

/// Implementation definition (`funcl` in Sail2 internals)
#[derive(Debug, Clone)]
pub struct ImplementationDefinition {}

/// Value definition
#[derive(Debug, Clone)]
pub struct LetBind {}

#[derive(Debug, Clone)]
pub struct RecMeasure {
    pub pattern: Pattern,
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub enum Funcls {
    PatExpTyp {},
    PatExp,
}

#[derive(Debug, Clone)]
pub enum InfixKind {
    Left,
    Right,
    None,
}

/// Top-level type constraint
#[derive(Debug, Clone)]
pub struct ValueSpec {}

#[derive(Debug, Clone)]
pub struct Expression;

#[derive(Debug, Clone)]
pub struct LoopMeasure;

#[derive(Debug, Clone)]
pub enum Direction {
    Inc,
    Dec,
}

#[derive(Debug, Clone)]
pub enum ScatteredDefinition {
    Union {
        id: String,
        type_parameter: Option<TypeParameter>,
    },
    Function {
        id: String,
    },
    Mapping {
        id: String,
        funcl_typ: Option<FunclTyp>,
    },
    FunctionClause {
        funcl: Funcl,
    },
    UnionClause {
        id: String,
        type_union: TypeUnion,
    },
    MappingClause {
        id: String,
        mapcl: MapCl,
    },
    End {
        id: String,
    },
}

#[derive(Debug, Clone)]
pub struct TypeParameter {}

#[derive(Debug, Clone)]
pub struct FunclTyp {}

#[derive(Debug, Clone)]
pub struct Funcl {}

#[derive(Debug, Clone)]
pub struct TypeUnion {}

#[derive(Debug, Clone)]
pub struct MapCl {}

#[derive(Debug, Clone)]
pub struct RegisterDefinition {
    pub effect_sets: Option<(EffectSet, EffectSet)>,
    pub configuration: bool,
    pub id: String,
    pub typ: TypeDefinition,
    pub expression: Option<Expression>,
}
#[derive(Debug, Clone)]
pub struct EffectSet {}

////////

#[derive(Debug, Clone)]
pub enum L {
    Unknown,
    Unique(i32, Box<L>),
    Generated(Box<L>),
    Range(Span),
    Documented(String, Box<L>),
}

#[derive(Debug, Clone)]
pub enum Kind {
    Int,
    Type,
    Order,
    Bool,
}

#[derive(Debug, Clone)]
pub enum BaseEffect {
    /// Read register
    ReadRegister,
    /// Write register
    WriteRegister,
    /// Read memory
    ReadMemory,
    /// Write memory
    WriteMemory,
    /// Write memory value
    WriteMemoryValue,
    /// Address for write signaled
    EaMemory,
    /// Determine if a store-exclusive (ARM) is going to succeed
    ExMemory,
    /// Memory barrier
    Barrier,
    /// Dynamically dependent footprint
    Depend,
    /// Undefined instruction exception
    Undefined,
    /// Unspecified values
    Unspecified,
    /// Nondeterminism from intra-instruction parallelism
    Nondetermine,
    ///
    Escape,
    ///
    Config,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Unit,
    Zero,
    One,
    True,
    False,
    Num(BigInt),
    Hex(String),
    Bin(String),
    Undefined,
    String(String),
    Real(String),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Wildcard,
    Or(Box<Pattern>, Box<Pattern>),
    Type(Atyp, Box<Pattern>),
    Identifier(String),
}

/// Expressions of all kinds, to be translated to types, nats, orders, and effects after parsing
#[derive(Debug, Clone)]
pub enum Atyp {
    /// Identifier
    Identifier(String),
    /// Ticked variable
    Variable(String),
    /// Literal
    Literal(Literal),
    /// Numeric set
    NumericSet {
        kid: String,
        body: Vec<BigInt>,
    },
    /// Product
    Product(Box<Atyp>, Box<Atyp>),
    /// Sum
    Sum(Box<Atyp>, Box<Atyp>),
    /// Subtraction
    Subtraction(Box<Atyp>, Box<Atyp>),
    /// Exponential
    Exponential(Box<Atyp>),
    /// Increasing
    Increasing,
    /// Decreasing
    Decreasing,
    /// Default order for increasing or decreasing signficant bits
    DefaultOrder,
    /// Effect set
    EffectSet(Vec<BaseEffect>),
    /// Function type, last element is an effect
    Function(Box<Atyp>, Box<Atyp>, Box<Atyp>),
    /// Mapping type, last element is an effect
    Mapping(Box<Atyp>, Box<Atyp>, Box<Atyp>),
    Wildcard,
    /// Tuple type
    Tuple(Vec<Atyp>),
    /// Type constructor application
    Application {
        id: String,
        body: Vec<Atyp>,
    },
    Exist(Vec<KindedIdentifier>, Box<Atyp>, Box<Atyp>),
    Base(String, Box<Atyp>, Box<Atyp>),
}

/// Optionally kind-annotated identifier
#[derive(Debug, Clone)]
pub struct KindedIdentifier(Option<String>, Vec<String>, Option<Kind>);

// /// Gets new instance of Sail parser
// pub fn parser() -> impl Parser<Token, Vec<Spanned<Definition>>, Error = Error> {
//     just(Token::Ampersand).to(Definition::InfixOperator)
// }

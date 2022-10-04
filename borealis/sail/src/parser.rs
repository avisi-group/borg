//! Main parser implementation and AST definition

use {
    crate::{
        error::Error,
        lexer::Token,
        span::{Span, Spanned},
    },
    chumsky::{
        combinator::To,
        primitive::{choice, just},
        select, Parser,
    },
    num_bigint::BigInt,
};

/// Gets new instance of Sail parser
pub fn parser<I>() -> impl Parser<Token, Vec<Spanned<Definition>>, Error = Error<Token>> {
    let pragma = select! { Token::Pragma(s) => Definition::Pragma(s.clone(), s)};

    let definition = choice((
        // function_definition,
        // mapping_definition,
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

    definition
        .map_with_span(|def, span| (def, Span::from(span)))
        .repeated()
}

#[derive(Debug, Clone)]
pub struct Extern {
    pub pure: bool,
    pub bindings: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Identifier(String),
    Operator(String),
}

/// Identifiers with kind, ticked to differntiate from program variables
#[derive(Debug, Clone)]
pub struct KindIdentifier(String);

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

/// Expressions of all kinds, to be translated to types, nats, orders, and effects after parsing
#[derive(Debug, Clone)]
pub enum Atyp {
    /// Identifier
    Identifier(Identifier),
    /// Ticked variable
    Variable(KindIdentifier),
    /// Literal
    Literal(Literal),
    /// Numeric set
    NumericSet(KindIdentifier, Vec<BigInt>),
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
    Application(Identifier, Vec<Atyp>),
    Exist(Vec<KindedIdentifier>, Box<Atyp>, Box<Atyp>),
    Base(String, Box<Atyp>, Box<Atyp>),
}

/// Optionally kind-annotated identifier
#[derive(Debug, Clone)]
pub struct KindedIdentifier(
    pub Option<String>,
    pub Vec<KindIdentifier>,
    pub Option<Kind>,
);

/// Either a kinded identifier or a nexp constraint for a typquant
#[derive(Debug, Clone)]
pub enum QuantItem {
    /// Optionally kinded identifier
    KindedIdentifier(KindedIdentifier),
    /// Constraint for this type
    Constraint(Atyp),
}

/// Type quantifiers and constraints
#[derive(Debug, Clone)]
pub enum TypQuant {
    Tq(Vec<QuantItem>),
    /// Sugar, omitting quantifier and constraints
    NoForAll,
}

/// Type scheme
#[derive(Debug, Clone)]
pub struct TypeScheme(pub TypQuant, pub Atyp);

/// Pattern
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Literal constant pattern
    Literal(Literal),
    /// Always matches
    Wildcard,
    /// Choice pattern
    ///
    /// `P|Q` matches if `P` matches OR `Q` matches
    Or(Box<Pattern>, Box<Pattern>),
    /// Typed pattern
    Type(Atyp, Box<Pattern>),
    /// Identifier
    Identifier(Identifier),
    /// Bind pattern to type variable
    Variable(Box<Pattern>, Atyp),
    /// Union constructor patern
    Application(Identifier, Vec<Pattern>),
    /// Vector pattern
    Vector(Vec<Pattern>),
    /// Concatenated vector pattern
    VectorConcat(Vec<Pattern>),
    /// Tuple pattern
    Tuple(Vec<Pattern>),
    /// List pattern
    List(Vec<Pattern>),
    /// Cons pattern
    Cons(Box<Pattern>, Box<Pattern>),
    /// String append pattern
    ///
    /// x^^y
    StringAppend(Vec<Pattern>),
}

/// Field pattern
#[derive(Debug, Clone)]
pub struct FieldPattern(pub Identifier, pub Pattern);

/// Loop kind
#[derive(Debug, Clone)]
pub enum Loop {
    While,
    Until,
}

/// Pattern match
///
/// `pexp` in Sail source
#[derive(Debug, Clone)]
pub enum PatternMatch {
    Expression(Pattern, Expression),
    When(Pattern, Expression, Expression),
}

/// Expression
#[derive(Debug, Clone)]
pub enum Expression {
    /// Block (parsing conflict with structs?)
    Block(Vec<Expression>),

    /// Identifier
    Identifier(Identifier),

    Ref(Identifier),

    Deref(Box<Expression>),

    /// Literal constant
    Literal(Literal),

    /// Cast
    Cast(Atyp, Box<Expression>),

    /// Function application
    Application(Identifier, Vec<Expression>),

    /// Infix function application
    ApplicationInfix(Box<Expression>, Identifier, Box<Expression>),

    /// Tuple
    Tuple(Vec<Expression>),

    /// Conditional
    If(Box<Expression>, Box<Expression>, Box<Expression>),

    Loop(
        Loop,
        Option<Box<Expression>>,
        Box<Expression>,
        Box<Expression>,
    ),

    For(
        Identifier,
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Atyp,
        Box<Expression>,
    ),

    /// Vector (indexed from 0)
    Vector(Vec<Expression>),

    /// Vector access
    VectorAccess(Box<Expression>, Box<Expression>),

    /// Subvector extraction
    VectorSubrange(Box<Expression>, Box<Expression>, Box<Expression>),

    /// Vector functional update
    VectorUpdate(Box<Expression>, Box<Expression>, Box<Expression>),

    /// Vector subrange update (with vector)
    VectorUpdateSubrange(
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
    ),

    /// Vector concatenation
    VectorAppend(Box<Expression>, Box<Expression>),

    /// List
    List(Vec<Expression>),

    /// Cons
    Cons(Box<Expression>, Box<Expression>),

    /// Struct
    Record(Vec<Expression>),

    /// Functional update of struct
    RecordUpdate(Box<Expression>, Vec<Expression>),

    /// Field projection from struct
    Field(Box<Expression>, Identifier),

    /// Pattern matching
    Case(Box<Expression>, Vec<PatternMatch>),

    /// Let expression
    Let(LetBind, Box<Expression>),

    /// Imperative assignment
    Assign(Box<Expression>, Box<Expression>),

    SizeOf(Atyp),

    Constraint(Atyp),

    Exit(Atyp),

    Throw(Atyp),

    Try(Box<Expression>, Vec<PatternMatch>),

    Return(Box<Expression>),

    Assert(Box<Expression>, Box<Expression>),

    Var(Box<Expression>, Box<Expression>, Box<Expression>),

    InternalPLet(Pattern, Box<Expression>, Box<Expression>),

    InternalReturn(Box<Expression>),
}

/// Field Expression
#[derive(Debug, Clone)]
pub struct FieldExpression(pub Identifier, pub Expression);

/// Optional default value for indexed vectors
///
/// To define a default value for any unspecified positions in a sparse map
#[derive(Debug, Clone)]
pub enum OptionalDefault {
    Empty,
    Dec(Expression),
}

/// Value binding
///
/// Implicit type, pattern must be total
#[derive(Debug, Clone)]
pub struct LetBind(pub Box<Pattern>, pub Box<Expression>);

/// Optional type annotation for functions
#[derive(Debug, Clone)]
pub struct TypeAnnotation(pub Option<(TypQuant, Atyp)>);

/// Optional effect annotation for functions
///
/// Inner `None` is sugar for empty effect set
#[derive(Debug, Clone)]
pub struct EffectAnnotation(pub Option<Atyp>);

/// Optional recursive annotation for functions
#[derive(Debug, Clone)]
pub enum RecursiveAnnotation {
    /// Non-recursive
    NonRecursive,
    /// Recursive
    Recursive,
    /// Recursive with termination measure
    Measure(Pattern, Expression),
}

/// Function clause
#[derive(Debug, Clone)]
pub struct FunctionClause(pub Identifier, pub PatternMatch);

/// Type union constructors
#[derive(Debug, Clone)]
pub enum TypeUnion {
    Identifier(Atyp, Identifier),
    AnonymousRecord(Vec<(Atyp, Identifier)>, Identifier),
}

/// Instantiation substitution
#[derive(Debug, Clone)]
pub enum Substitution {
    /// Instantiate a type variable with a type
    Type(KindIdentifier, Atyp),
    /// Instantiate an identifier with another identifier
    Identifier(Identifier, Identifier),
}

/// Index specification, for bitfields in register types
#[derive(Debug, Clone)]
pub enum IndexRange {
    /// Single index
    Single(Atyp),
    /// Index range
    Range(Atyp, Atyp),
    /// Concatenation of index ranges
    Concat(Box<IndexRange>, Box<IndexRange>),
}

/// Default kinding or typing assumption, and default order for literal vectors and vector shorthands
#[derive(Debug, Clone)]
pub struct DefaultTypingSpecification(pub Kind, pub Atyp);

/// Mapping pattern
///
/// Mostly the same as normal patterns but only constructible parts
#[derive(Debug, Clone)]
pub enum MappingPattern {
    /// Literal
    Literal(Literal),
    Identifier(Identifier),
    /// Union constructor patern
    Application(Identifier, Vec<MappingPattern>),
    /// Vector pattern
    Vector(Vec<MappingPattern>),
    /// Concatenated vector pattern
    VectorConcat(Vec<MappingPattern>),
    /// Tuple pattern
    Tuple(Vec<MappingPattern>),
    /// List pattern
    List(Vec<MappingPattern>),
    /// Cons pattern
    Cons(Box<MappingPattern>, Box<MappingPattern>),
    /// String append pattern
    ///
    /// x^^y
    StringAppend(Vec<MappingPattern>),
    /// Typed pattern
    Type(Atyp, Box<MappingPattern>),
    As(Box<MappingPattern>, Identifier),
}

/// Mapping pattern expression
#[derive(Debug, Clone)]
pub enum MappingPatternExpression {
    Pattern(MappingPattern),
    When(MappingPattern, Expression),
}

/// Mapping clause (bidirectional pattern-match)
#[derive(Debug, Clone)]
pub enum MappingClause {
    Bidirectional(MappingPatternExpression, MappingPatternExpression),
    Forwards(MappingPatternExpression, MappingPatternExpression),
    Backwards(MappingPatternExpression, MappingPatternExpression),
}

/// Mapping definition (bidirectional pattern-match function)
#[derive(Debug, Clone)]
pub struct MappingDefinition(
    pub Identifier,
    pub Option<TypeScheme>,
    pub Vec<MappingClause>,
);

/// Outcome declaration
#[derive(Debug, Clone)]
pub struct OutcomeDeclaration(pub Identifier, pub TypeScheme, pub Vec<KindedIdentifier>);

/// Function definition
#[derive(Debug, Clone)]
pub struct FunctionDefinition(
    pub RecursiveAnnotation,
    pub TypeAnnotation,
    pub EffectAnnotation,
    pub Vec<FunctionClause>,
);

/// Type definition body
#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Abbreviation(Identifier, TypQuant, Kind, Atyp),
    /// Struct type definition
    Record(Identifier, TypQuant, Vec<(Atyp, Identifier)>, bool),
    /// Union type definition
    Variant(Identifier, TypQuant, Vec<TypeUnion>, bool),
    /// Enumeration type definition
    Enumeration(
        Identifier,
        Vec<(Identifier, Atyp)>,
        Vec<(Identifier, Option<Expression>)>,
        bool,
    ),
    /// Register mutable bitfield type definition
    Bitfield(Identifier, Atyp, Vec<(Identifier, IndexRange)>),
}

/// Value type specification
#[derive(Debug, Clone)]
pub struct ValueSpecification(TypeScheme, Identifier, Option<Extern>, bool);

/// Register declarations
#[derive(Debug, Clone)]
pub enum DecSpec {
    Register(Atyp, Atyp, Atyp, Identifier, Option<Expression>),
    Config(Identifier, Atyp, Expression),
}

/// Function and type union definitions that can be spread across a file. Each one must end in $_$
#[derive(Debug, Clone)]
pub enum ScatteredDefinition {
    /// Scattered function definition header
    Function(
        RecursiveAnnotation,
        TypeAnnotation,
        EffectAnnotation,
        Identifier,
    ),
    /// Scattered function definition clause
    FunctionClause(FunctionClause),
    /// Scattered union definition header
    Variant(Identifier, TypQuant),
    /// Scattered union definition member
    UnionClause(Identifier, TypeUnion),
    Mapping(Identifier, TypeAnnotation),
    MappingClause(Identifier, MappingClause),
    /// Scattered definition end
    End(Identifier),
}

#[derive(Debug, Clone)]
pub struct LoopMeasure(Loop, Expression);

#[derive(Debug, Clone)]
pub enum Prec {
    Infix,
    InfixLeft,
    InfixRight,
}

#[derive(Debug, Clone)]
pub struct FixityToken(Prec, BigInt, String);

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
    Implementation(FunctionClause),

    /// Let bind value definition
    Let(LetBind),

    /// Operator overload specifications
    Overload {
        identifier: Identifier,
        ids: Vec<Identifier>,
    },

    /// Fixity declaration
    InfixOperator {
        kind: Prec,
        digit: BigInt,
        operator: Identifier,
    },

    /// Top-level type constraint
    ValueSpec(ValueSpecification),

    /// Top-level outcome definition
    OutcomeSpec(OutcomeDeclaration, Vec<Definition>),

    /// Instantiation
    Instantiation {
        identifier: Identifier,
        subst: Vec<Substitution>,
    },

    /// Default type and kind assumptions
    Default(DefaultTypingSpecification),

    Scattered(ScatteredDefinition),

    /// Separate termination measure declaration
    TerminationMeasurePatternExpression {
        identifier: Identifier,
        pattern: Pattern,
        expression: Expression,
    },

    /// Separate termination measure declaration
    TerminationMeasureLoop {
        identifier: Identifier,
        loop_measure: Vec<LoopMeasure>,
    },

    /// Register declaration
    Register(DecSpec),

    /// Pragma
    Pragma(String, String),

    /// Internal mutrec
    Mutual(Vec<FunctionDefinition>),
}

/// l-value expression
#[derive(Debug, Clone)]
pub enum LValueExpression {
    Identifier(Identifier),
    Memory(Identifier, Vec<Expression>),
    Vector(Box<LValueExpression>, Expression),
    VectorRange(Box<LValueExpression>, Expression, Expression),
    VectorConcat(Vec<LValueExpression>),
    Field(Box<LValueExpression>, Identifier),
}

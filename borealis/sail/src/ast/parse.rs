//! Parsed, processed, and desugared AST generated from `l2_parse.ott`.

use {
    crate::ffi::{BigNum, OCamlString, Position},
    ocaml::{FromValue, Int},
    std::{collections::LinkedList, fmt::Debug},
};

pub type Text = OCamlString;

/// Location
#[derive(Debug, Clone, FromValue)]
pub enum L {
    /// Unknown location
    Unknown,
    /// Unique location
    Unique(Int, Box<L>),
    /// Generated location
    Generated(Box<L>),
    /// Range between two positions
    Range(Position, Position),
    /// Documented location
    Documented(OCamlString, Box<L>),
}

/// Idenitifer
pub type X = Text;

/// Infix identifier
pub type Xi = Text;

/// Base kind
#[derive(Debug, Clone, FromValue)]
pub enum KindAux {
    /// Kind of types
    Type,
    /// Kind of natural number size expressions
    Int,
    /// Kind of vector order specifications
    Order,
    /// Kind of constraints
    Bool,
}

/// Base kind
#[derive(Debug, Clone, FromValue)]
pub struct Kind(KindAux, L);

#[derive(Debug, Clone, FromValue)]
pub enum BaseEffectAux {
    /// Read register
    ReadRegister,
    /// Write register
    WriteRegister,
    /// Read memory
    ReadMemory,
    /// Read memory tagged
    ReadMemoryTagged,
    /// Write memory
    WriteMemory,
    /// Write memory value
    WriteMemoryValue,
    /// Write memory value tagged
    WriteMemoryValueTagged,
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
    /// Potential exception
    Escape,
    /// Configuration option
    Config,
}

/// Identifiers with kind, ticked to differntiate from program variables
#[derive(Debug, Clone, FromValue)]
pub enum KindIdentifierAux {
    Var(X),
}

#[derive(Debug, Clone, FromValue)]
pub enum IdentifierAux {
    Identifier(X),
    Operator(X),
}

#[derive(Debug, Clone, FromValue)]
pub struct BaseEffect(BaseEffectAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct KindIdentifier(KindIdentifierAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct Identifier(IdentifierAux, L);

#[derive(Debug, Clone, FromValue)]
pub enum LiteralAux {
    Unit,
    Zero,
    One,
    True,
    False,
    /// Natural number constant
    Num(BigNum),
    /// Bit vector constant, C-style
    Hex(OCamlString),
    /// Bit vector constant, C-style
    Bin(OCamlString),
    /// Undefined value
    Undefined,
    /// String constant
    String(OCamlString),
    Real(OCamlString),
}

#[derive(Debug, Clone, FromValue)]
pub struct Literal(LiteralAux, L);

/// Expressions of all kinds, to be translated to types, nats, orders, and effects after parsing
#[derive(Debug, Clone, FromValue)]
pub enum AtypAux {
    /// Identifier
    Identifier(Identifier),
    /// Ticked variable
    Variable(KindIdentifier),
    /// Literal
    Literal(Literal),
    /// Numeric set
    NumericSet(KindIdentifier, LinkedList<BigNum>),
    /// Product
    Product(Box<Atyp>, Box<Atyp>),
    /// Sum
    Sum(Box<Atyp>, Box<Atyp>),
    /// Subtraction
    Subtraction(Box<Atyp>, Box<Atyp>),
    /// Exponential
    Exponential(Box<Atyp>),
    /// (* Internal (but not M as I want a datatype constructor) negative nexp
    NegativeExponential(Box<Atyp>),
    /// Increasing
    Increasing,
    /// Decreasing
    Decreasing,
    /// Default order for increasing or decreasing signficant bits
    DefaultOrder,
    /// Effect set
    EffectSet(LinkedList<BaseEffect>),
    /// Function type, last element is an effect
    Function(Box<Atyp>, Box<Atyp>, Box<Atyp>),
    /// Mapping type, last element is an effect
    Mapping(Box<Atyp>, Box<Atyp>, Box<Atyp>),
    Wildcard,
    /// Tuple type
    Tuple(LinkedList<Atyp>),
    /// Type constructor application
    Application(Identifier, LinkedList<Atyp>),
    Exist(LinkedList<KindedIdentifier>, Box<Atyp>, Box<Atyp>),
    Base(OCamlString, Box<Atyp>, Box<Atyp>),
}

/// Expressions of all kinds, to be translated to types, nats, orders, and effects after parsing
#[derive(Debug, Clone, FromValue)]
pub struct Atyp(AtypAux, L);

/// Optionally kind-annotated identifier
#[derive(Debug, Clone, FromValue)]
pub struct KindedIdentifierAux(
    pub Option<OCamlString>,
    pub LinkedList<KindIdentifier>,
    pub Option<Kind>,
);

/// Optionally kind-annotated identifier
#[derive(Debug, Clone, FromValue)]
pub struct KindedIdentifier(KindIdentifierAux, L);

/// Either a kinded identifier or a nexp constraint for a typquant
#[derive(Debug, Clone, FromValue)]
pub enum QuantItemAux {
    /// Optionally kinded identifier
    KindedIdentifier(KindedIdentifier),
    /// Constraint for this type
    Constraint(Atyp),
}

/// Either a kinded identifier or a nexp constraint for a typquant
#[derive(Debug, Clone, FromValue)]
pub struct QuantItem(QuantItemAux, L);

/// Type quantifiers and constraints
#[derive(Debug, Clone, FromValue)]
pub enum TypQuantAux {
    Tq(LinkedList<QuantItem>),
    /// Sugar, omitting quantifier and constraints
    NoForAll,
}

#[derive(Debug, Clone, FromValue)]
pub struct TypQuant(TypQuantAux, L);

/// Type scheme
#[derive(Debug, Clone, FromValue)]
pub struct TypeSchemeAux(pub TypQuant, pub Atyp);

#[derive(Debug, Clone, FromValue)]
pub struct TypeScheme(TypeSchemeAux, L);

/// Pattern
#[derive(Debug, Clone, FromValue)]
pub enum PatternAux {
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
    Application(Identifier, LinkedList<Pattern>),
    /// Vector pattern
    Vector(LinkedList<Pattern>),
    /// Concatenated vector pattern
    VectorConcat(LinkedList<Pattern>),
    /// Tuple pattern
    Tuple(LinkedList<Pattern>),
    /// List pattern
    List(LinkedList<Pattern>),
    /// Cons pattern
    Cons(Box<Pattern>, Box<Pattern>),
    /// String append pattern
    ///
    /// x^^y
    StringAppend(LinkedList<Pattern>),
}

/// Pattern
#[derive(Debug, Clone, FromValue)]
pub struct Pattern(PatternAux, L);

/// Field pattern
#[derive(Debug, Clone, FromValue)]
pub struct FieldPatternAux(pub Identifier, pub Pattern);

/// Field pattern
#[derive(Debug, Clone, FromValue)]
pub struct FieldPattern(FieldPatternAux, L);

/// Loop kind
#[derive(Debug, Clone, FromValue)]
pub enum Loop {
    While,
    Until,
}

/// Optional termination measure for a loop
#[derive(Debug, Clone, FromValue)]
pub enum MeasureAux {
    None,
    Some(Expression),
}

/// Optional termination measure for a loop
#[derive(Debug, Clone, FromValue)]
pub struct Measure(MeasureAux, L);

/// Expression
#[derive(Debug, Clone, FromValue)]
pub enum ExpressionAux {
    /// Block (parsing conflict with structs?)
    Block(LinkedList<Expression>),

    /// Identifier
    Identifier(Identifier),

    Ref(Identifier),

    Deref(Box<Expression>),

    /// Literal constant
    Literal(Literal),

    /// Cast
    Cast(Atyp, Box<Expression>),

    /// Function application
    Application(Identifier, LinkedList<Expression>),

    /// Infix function application
    ApplicationInfix(Box<Expression>, Identifier, Box<Expression>),

    /// Tuple
    Tuple(LinkedList<Expression>),

    /// Conditional
    If(Box<Expression>, Box<Expression>, Box<Expression>),

    Loop(Loop, Box<Measure>, Box<Expression>, Box<Expression>),

    For(
        Identifier,
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Atyp,
        Box<Expression>,
    ),

    /// Vector (indexed from 0)
    Vector(LinkedList<Expression>),

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
    List(LinkedList<Expression>),

    /// Cons
    Cons(Box<Expression>, Box<Expression>),

    /// Struct
    Record(LinkedList<Expression>),

    /// Functional update of struct
    RecordUpdate(Box<Expression>, LinkedList<Expression>),

    /// Field projection from struct
    Field(Box<Expression>, Identifier),

    /// Pattern matching
    Case(Box<Expression>, LinkedList<PatternMatch>),

    /// Let expression
    Let(LetBind, Box<Expression>),

    /// Imperative assignment
    Assign(Box<Expression>, Box<Expression>),

    SizeOf(Atyp),

    Constraint(Atyp),

    Exit(Box<Expression>),

    Throw(Box<Expression>),

    Try(Box<Expression>, LinkedList<PatternMatch>),

    Return(Box<Expression>),

    Assert(Box<Expression>, Box<Expression>),

    Var(Box<Expression>, Box<Expression>, Box<Expression>),

    InternalPLet(Pattern, Box<Expression>, Box<Expression>),

    InternalReturn(Box<Expression>),
}

/// Expression
#[derive(Debug, Clone, FromValue)]
pub struct Expression(ExpressionAux, L);

/// Field Expression
#[derive(Debug, Clone, FromValue)]
pub struct FieldExpressionAux(pub Identifier, pub Expression);

/// Field Expression
#[derive(Debug, Clone, FromValue)]
pub struct FieldExpression(FieldExpressionAux, L);

/// Optional default value for indexed vectors
///
/// To define a default value for any unspecified positions in a sparse map
#[derive(Debug, Clone, FromValue)]
pub enum OptionalDefaultAux {
    Empty,
    Dec(Expression),
}

/// Optional default value for indexed vectors
///
/// To define a default value for any unspecified positions in a sparse map
#[derive(Debug, Clone, FromValue)]
pub struct OptionalDefault(OptionalDefaultAux, L);

/// Pattern match
///
/// `pexp` in Sail source
#[derive(Debug, Clone, FromValue)]
pub enum PatternMatchAux {
    Expression(Pattern, Expression),
    When(Pattern, Expression, Expression),
}

/// Pattern match
///
/// `pexp` in Sail source
#[derive(Debug, Clone, FromValue)]
pub struct PatternMatch(PatternMatchAux, L);

/// Value binding
///
/// Implicit type, pattern must be total
#[derive(Debug, Clone, FromValue)]
pub struct LetBindAux(pub Box<Pattern>, pub Box<Expression>);

#[derive(Debug, Clone, FromValue)]
pub struct LetBind(LetBindAux, L);

/// Optional type annotation for functions
#[derive(Debug, Clone, FromValue)]
pub enum TypeAnnotationOptAux {
    None,
    Some(TypQuant, Atyp),
}

#[derive(Debug, Clone, FromValue)]
pub enum TypeSchemeOptAux {
    None,
    Some(TypeScheme),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypeSchemeOpt(TypeSchemeOptAux, L);

/// Optional effect annotation for functions
#[derive(Debug, Clone, FromValue)]
pub enum EffectAnnotationOptAux {
    /// Sugar for empty effect set
    None,
    Some(Atyp),
}

/// Optional recursive annotation for functions
#[derive(Debug, Clone, FromValue)]
pub enum RecursiveAnnotationOptAux {
    /// Non-recursive
    NonRecursive,
    /// Recursive
    Recursive,
    /// Recursive with termination measure
    Measure(Pattern, Expression),
}

/// Function clause
#[derive(Debug, Clone, FromValue)]
pub struct FunctionClauseAux(pub Identifier, pub PatternMatch);

/// Type union constructors
#[derive(Debug, Clone, FromValue)]
pub enum TypeUnionAux {
    Identifier(Atyp, Identifier),
    AnonymousRecord(LinkedList<(Atyp, Identifier)>, Identifier),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypeAnnotationOpt(TypeAnnotationOptAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct EffectAnnotationOpt(EffectAnnotationOptAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct RecursiveAnnotationOpt(RecursiveAnnotationOptAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct FunctionClause(FunctionClauseAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct TypeUnion(TypeUnionAux, L);

/// Index specification, for bitfields in register types
#[derive(Debug, Clone, FromValue)]
pub enum IndexRangeAux {
    /// Single index
    Single(Atyp),
    /// Index range
    Range(Atyp, Atyp),
    /// Concatenation of index ranges
    Concat(Box<IndexRange>, Box<IndexRange>),
}

#[derive(Debug, Clone, FromValue)]
pub struct IndexRange(IndexRangeAux, L);

/// Default kinding or typing assumption, and default order for literal vectors and vector shorthands
#[derive(Debug, Clone, FromValue)]
pub struct DefaultTypingSpecificationAux(pub Kind, pub Atyp);

/// Mapping pattern
///
/// Mostly the same as normal patterns but only constructible parts
#[derive(Debug, Clone, FromValue)]
pub enum MappingPatternAux {
    /// Literal
    Literal(Literal),
    Identifier(Identifier),
    /// Union constructor patern
    Application(Identifier, LinkedList<MappingPattern>),
    /// Vector pattern
    Vector(LinkedList<MappingPattern>),
    /// Concatenated vector pattern
    VectorConcat(LinkedList<MappingPattern>),
    /// Tuple pattern
    Tuple(LinkedList<MappingPattern>),
    /// List pattern
    List(LinkedList<MappingPattern>),
    /// Cons pattern
    Cons(Box<MappingPattern>, Box<MappingPattern>),
    /// String append pattern
    ///
    /// x^^y
    StringAppend(LinkedList<MappingPattern>),
    /// Typed pattern
    Type(Box<MappingPattern>, Atyp),
    As(Box<MappingPattern>, Identifier),
}

#[derive(Debug, Clone, FromValue)]
pub struct MappingPattern(MappingPatternAux, L);

/// Mapping pattern expression
#[derive(Debug, Clone, FromValue)]
pub enum MappingPatternExpressionAux {
    Pattern(MappingPattern),
    When(MappingPattern, Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct MappingPatternExpression(MappingPatternExpressionAux, L);

/// Mapping clause (bidirectional pattern-match)
#[derive(Debug, Clone, FromValue)]
pub enum MappingClauseAux {
    Bidirectional(MappingPatternExpression, MappingPatternExpression),
    Forwards(MappingPatternExpression, Expression),
    Backwards(MappingPatternExpression, Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct MappingClause(MappingClauseAux, L);

/// Mapping definition (bidirectional pattern-match function)
#[derive(Debug, Clone, FromValue)]
pub struct MappingDefinitionAux(
    pub Identifier,
    pub TypeSchemeOpt,
    pub LinkedList<MappingClause>,
);

#[derive(Debug, Clone, FromValue)]
pub struct MappingDefinition(MappingDefinitionAux, L);

/// Function definition
#[derive(Debug, Clone, FromValue)]
pub struct FunctionDefinitionAux(
    pub RecursiveAnnotationOpt,
    pub TypeAnnotationOpt,
    pub EffectAnnotationOpt,
    pub LinkedList<FunctionClause>,
);

/// Type definition body
#[derive(Debug, Clone, FromValue)]
pub enum TypeDefinitionAux {
    /// Type abbreviation
    Abbreviation(Identifier, TypQuant, Kind, Atyp),
    /// Struct type definition
    Record(Identifier, TypQuant, LinkedList<(Atyp, Identifier)>, bool),
    /// Union type definition
    Variant(Identifier, TypQuant, LinkedList<TypeUnion>, bool),
    /// Enumeration type definition
    Enumeration(
        Identifier,
        LinkedList<(Identifier, Atyp)>,
        LinkedList<(Identifier, Option<Expression>)>,
        bool,
    ),
    /// Register mutable bitfield type definition
    Bitfield(Identifier, Atyp, LinkedList<(Identifier, IndexRange)>),
}

/// Value type specification
#[derive(Debug, Clone, FromValue)]
pub struct ValueSpecificationAux(
    TypeScheme,
    Identifier,
    LinkedList<(OCamlString, OCamlString)>,
    bool,
);

/// Register declarations
#[derive(Debug, Clone, FromValue)]
pub enum DecSpecAux {
    Register(Atyp, Atyp, Atyp, Identifier),
    Config(Identifier, Atyp, Expression),
    Alias(Identifier, Expression),
    TypeAlias(Atyp, Identifier, Expression),
}

/// Function and type union definitions that can be spread across a file. Each one must end in $_$
#[derive(Debug, Clone, FromValue)]
pub enum ScatteredDefinitionAux {
    /// Scattered function definition header
    Function(
        RecursiveAnnotationOpt,
        TypeAnnotationOpt,
        EffectAnnotationOpt,
        Identifier,
    ),
    /// Scattered function definition clause
    FunctionClause(FunctionClause),
    /// Scattered union definition header
    Variant(Identifier, TypQuant),
    /// Scattered union definition member
    UnionClause(Identifier, TypeUnion),
    Mapping(Identifier, TypeAnnotationOpt),
    MappingClause(Identifier, MappingClause),
    /// Scattered definition end
    End(Identifier),
}

#[derive(Debug, Clone, FromValue)]
pub struct DefaultTypingSpecification(DefaultTypingSpecificationAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct FunctionDefinition(FunctionDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct TypeDefinition(TypeDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct ValueSpecification(ValueSpecificationAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct DecSpec(DecSpecAux, L);

#[derive(Debug, Clone, FromValue)]
pub enum LoopMeasure {
    Loop(Loop, Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct ScatteredDefinition(ScatteredDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub enum Prec {
    Infix,
    InfixLeft,
    InfixRight,
}

#[derive(Debug, Clone, FromValue)]
pub struct FixityToken(Prec, BigNum, OCamlString);

/// Top-level Sail2 definition
#[derive(Debug, Clone, FromValue)]
pub enum Definition {
    /// Type definition
    Type(TypeDefinition),

    /// Function definition
    Function(FunctionDefinition),

    /// Mapping definition
    Mapping(MappingDefinition),

    /// Value definition
    Value(LetBind),

    /// Operator overload specifications
    Overload(Identifier, LinkedList<Identifier>),

    /// Fixity declaration
    Fixity(Prec, BigNum, Identifier),

    /// Top-level type constraint
    ValueSpec(ValueSpecification),

    /// Default type and kind assumptions
    Default(DefaultTypingSpecification),

    /// Scattered definition
    Scattered(ScatteredDefinition),

    /// Separate termination measure declaration
    TerminationMeasurePatternExpression {
        identifier: Identifier,
        pattern: Pattern,
        expression: Expression,
    },

    /// Separate termination measure declaration
    TerminationMeasureLoop(Identifier, LinkedList<LoopMeasure>),

    /// Register declaration
    Register(DecSpec),

    /// Pragma
    Pragma(OCamlString, OCamlString, L),

    /// Internal mutrec
    Mutual(LinkedList<FunctionDefinition>),
}

/// l-value expression
#[derive(Debug, Clone, FromValue)]
pub enum LValueExpressionAux {
    Identifier(Identifier),
    Memory(Identifier, LinkedList<Expression>),
    Vector(Box<LValueExpression>, Expression),
    VectorRange(Box<LValueExpression>, Expression, Expression),
    VectorConcat(LinkedList<LValueExpression>),
    Field(Box<LValueExpression>, Identifier),
}

#[derive(Debug, Clone, FromValue)]
pub struct LValueExpression(LValueExpressionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct Definitions(LinkedList<(String, LinkedList<Definition>)>);

#![allow(missing_docs)]

//! Sail Abstract Syntax Tree

pub mod type_check;

use {
    ocaml::{FromValue, Int, Value},
    std::{collections::LinkedList, fmt::Debug},
};

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
    Documented(String, Box<L>),
}

/// Position in a source file
#[derive(Debug, Clone, FromValue)]
pub struct Position {
    /// File name
    pub pos_fname: String,
    /// Line number
    pub pos_lnum: Int,
    /// Character offset of beginning of line
    pub pos_bol: Int,
    /// Character offset of the position
    pub pos_cnum: Int,
}

pub type Text = String;

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
    /// ?
    Escape,
    /// ?
    Config,
}

#[derive(Debug, Clone, FromValue)]
pub struct BaseEffect(BaseEffectAux, L);

/// Identifiers with kind, ticked to differntiate from program variables
#[derive(Debug, Clone, FromValue)]
pub enum KindIdentifierAux {
    Var(X),
}

#[derive(Debug, Clone, FromValue)]
pub struct KindIdentifier(KindIdentifierAux, L);

#[derive(Debug, Clone, FromValue)]
pub enum IdentifierAux {
    Identifier(X),
    Operator(X),
}

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
    Num(Value),
    /// Bit vector constant, C-style
    Hex(String),
    /// Bit vector constant, C-style
    Bin(String),
    /// Undefined value
    Undefined,
    /// String constant
    String(String),
    Real(String),
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
    NumericSet(KindIdentifier, LinkedList<Value>),
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
    Base(String, Box<Atyp>, Box<Atyp>),
}

/// Expressions of all kinds, to be translated to types, nats, orders, and effects after parsing
#[derive(Debug, Clone, FromValue)]
pub struct Atyp(AtypAux, L);

/// Optionally kind-annotated identifier
#[derive(Debug, Clone, FromValue)]
pub struct KindedIdentifierAux(
    pub Option<String>,
    pub LinkedList<KindIdentifier>,
    pub Option<KindAux>,
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

    Exit(Atyp),

    Throw(Atyp),

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
pub struct TypeAnnotationOpt(TypeAnnotationOptAux, L);

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

#[derive(Debug, Clone, FromValue)]
pub struct EffectAnnotationOpt(EffectAnnotationOptAux, L);

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

#[derive(Debug, Clone, FromValue)]
pub struct RecursiveAnnotationOpt(RecursiveAnnotationOptAux, L);

/// Function clause
#[derive(Debug, Clone, FromValue)]
pub struct FunctionClauseAux(pub Identifier, pub PatternMatch);

#[derive(Debug, Clone, FromValue)]
pub struct FunctionClause(FunctionClauseAux, L);

/// Type union constructors
#[derive(Debug, Clone, FromValue)]
pub enum TypeUnionAux {
    Identifier(Atyp, Identifier),
    AnonymousRecord(LinkedList<(Atyp, Identifier)>, Identifier),
}

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

#[derive(Debug, Clone, FromValue)]
pub struct DefaultTypingSpecification(DefaultTypingSpecificationAux, L);

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
    pub Option<TypeScheme>,
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

#[derive(Debug, Clone, FromValue)]
pub struct FunctionDefinition(FunctionDefinitionAux, L);

/// Type definition body
#[derive(Debug, Clone, FromValue)]
pub enum TypeDefinitionAux {
    Abbreviation(Identifier, Value, Value),
    /// Struct type definition
    Record(Value),
    /// Union type definition
    Variant(Value),
    /// Enumeration type definition
    Enumeration(Value),
    /// Register mutable bitfield type definition
    Bitfield(Value),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypeDefinition(TypeDefinitionAux, L);

/// Value type specification
#[derive(Debug, Clone, FromValue)]
pub struct ValueSpecificationAux(TypeScheme, Identifier, LinkedList<(String, String)>, bool);

#[derive(Debug, Clone, FromValue)]
pub struct ValueSpecification(ValueSpecificationAux, L);

/// Register declarations
#[derive(Debug, Clone, FromValue)]
pub enum DecSpecAux {
    Register(Value, Value, Value, Identifier),
    Config(Identifier, Atyp, Expression),
    Alias(Identifier, Value),
    TypeAlias(Value, Identifier, Value),
}

#[derive(Debug, Clone, FromValue)]
pub struct DecSpec(DecSpecAux, L);

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
pub struct ScatteredDefinition(ScatteredDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct LoopMeasure(Loop, Expression);

#[derive(Debug, Clone, FromValue)]
pub enum Prec {
    Infix,
    InfixLeft,
    InfixRight,
}

#[derive(Debug, Clone, FromValue)]
pub struct FixityToken(Prec, Value, String);

/// Top-level Sail2 definition
#[derive(Debug, Clone, FromValue)]
pub enum Definition {
    /// Type definition
    Type((L, Value)),

    /// Function definition
    Function(Value),

    /// Mapping definition
    Mapping(Value),

    /// Value definition
    Value(Value),

    /// Operator overload specifications
    Overload(Value),

    /// Fixity declaration
    Fixity(Prec, Value, Identifier),

    /// Top-level type constraint
    ValueSpec(Value),

    /// Implementation definition (`funcl` in Sail2 internals)
    Implementation(Value),

    /// Fixity declaration
    InfixOperator {
        kind: Prec,
        digit: Value,
        operator: Identifier,
    },

    /// Default type and kind assumptions
    Default(Value),

    /// Scattered definition
    Scattered(Value),

    /// Separate termination measure declaration
    TerminationMeasurePatternExpression {
        identifier: Identifier,
        pattern: Pattern,
        expression: Expression,
    },

    /// Separate termination measure declaration
    TerminationMeasureLoop(Value),

    /// Register declaration
    Register(Value),

    /// Pragma
    Pragma(String, String),

    /// Internal mutrec
    Mutual(LinkedList<Value>),
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

// #[derive(Debug, Clone, FromValue)]
// pub struct Definitions(LinkedList<(String, LinkedList<Definition>)>);

#[derive(Debug, Clone, FromValue)]
pub enum CommentType {
    Block,
    Line,
}

#[derive(Debug, Clone, FromValue)]

pub struct Comment(CommentType, Position, Position, String);

#[derive(Debug, Clone, FromValue)]
pub struct Ast {
    pub defs: LinkedList<Definition>,
    pub comments: LinkedList<(String, LinkedList<Comment>)>,
}

unsafe impl Send for Ast {}
unsafe impl Sync for Ast {}

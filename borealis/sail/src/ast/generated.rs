//! Generated AST from `ast.lem` and `sail.ott`.

use {
    crate::{
        ast::{parse::L, Value},
        ffi::{BigNum, OCamlString, Position},
    },
    ocaml::FromValue,
    std::{collections::LinkedList, fmt::Debug},
};

/// Loop kind
#[derive(Debug, Clone, FromValue)]
pub enum Loop {
    While,
    Until,
}

/// Idenitifer
pub type X = OCamlString;

/// Infix identifier
pub type Xi = OCamlString;

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
    /// Address for write signaled
    EaMemory,
    /// Determine if a store-exclusive (ARM) is going to succeed
    ExMemory,
    /// Write memory value
    WriteMemoryValue,
    /// Write memory value tagged
    WriteMemoryValueTagged,
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

/// kinded IDs: Type, Int, and Order variables
#[derive(Debug, Clone, FromValue)]
pub enum KindIdentifierAux {
    Var(X),
}

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

#[derive(Debug, Clone, FromValue)]
pub enum IdentifierAux {
    Identifier(X),
    Operator(X),
}

#[derive(Debug, Clone, FromValue)]
pub struct BaseEffect(BaseEffectAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct KindIdentifier(KindIdentifierAux, L);

/// Base kind
#[derive(Debug, Clone, FromValue)]
pub struct Kind(KindAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct Identifier(IdentifierAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct EffectAux(LinkedList<BaseEffect>);

/// Vector order specifications, of kind Order
#[derive(Debug, Clone, FromValue)]
pub enum OrderAux {
    Variable(KindIdentifier),
    Increasing,
    Decreasing,
}

/// Optionally kind-annotated identifier
#[derive(Debug, Clone, FromValue)]
pub struct KindedIdentifierAux(Kind, KindIdentifier);

/// Numeric expression, of kind Int
#[derive(Debug, Clone, FromValue)]
pub enum NumericExpressionAux {
    /// Abbreviation identifier
    Id(Identifier),
    /// Variable
    Var(KindIdentifier),
    /// Constant
    Constant(ocaml::Value),
    Application(Identifier, LinkedList<NumericExpression>),
    Times(Box<NumericExpression>, Box<NumericExpression>),
    Sum(Box<NumericExpression>, Box<NumericExpression>),
    Minus(Box<NumericExpression>, Box<NumericExpression>),
    Exponential(Box<NumericExpression>),
    /// Unary negation
    Negation(Box<NumericExpression>),
}

#[derive(Debug, Clone, FromValue)]
pub struct NumericExpression(NumericExpressionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct Effect(EffectAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct Order(OrderAux, L);

/// Optionally kind-annotated identifier
#[derive(Debug, Clone, FromValue)]
pub struct KindedIdentifier(KindIdentifierAux, L);

/// Literal constant
#[derive(Debug, Clone, FromValue)]
pub enum LiteralAux {
    Unit,
    Zero,
    One,
    True,
    False,
    /// Natural number constant
    Num(ocaml::Value),
    /// Bit vector constant, C-style
    Hex(OCamlString),
    /// Bit vector constant, C-style
    Bin(OCamlString),
    /// String constant
    String(OCamlString),
    /// Undefined value constant
    Undefined,
    Real(OCamlString),
}

/// Type expressions, of kind Type
#[derive(Debug, Clone, FromValue)]
pub enum TypAux {
    InternalUnknown,
    /// Defined type
    Id(Identifier),
    /// Type variable
    Var(KindIdentifier),
    /// Function (first-order only)
    Fn(LinkedList<Typ>, Box<Typ>, Effect),
    /// Mapping
    BiDir(Box<Typ>, Box<Typ>, Effect),
    Tuple(LinkedList<Typ>),
    /// Type constructor application
    Application(Identifier, LinkedList<TypArg>),
    Exist(LinkedList<KindedIdentifier>, NConstraint, Box<Typ>),
}

#[derive(Debug, Clone, FromValue)]
pub struct Typ(TypAux, L);

/// Type constructor arguments of all kinds
#[derive(Debug, Clone, FromValue)]
pub enum TypArgAux {
    NExp(NumericExpression),
    Typ(Typ),
    Order(Order),
    Bool(NConstraint),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypArg(TypArgAux, L);

/// Constraint over kind Int
#[derive(Debug, Clone, FromValue)]
pub enum NConstraintAux {
    Equal(NumericExpression, NumericExpression),
    BoundedGe(NumericExpression, NumericExpression),
    BoundedGt(NumericExpression, NumericExpression),
    BoundedLe(NumericExpression, NumericExpression),
    BoundedLt(NumericExpression, NumericExpression),
    NotEqual(NumericExpression, NumericExpression),
    Set(KindIdentifier, LinkedList<ocaml::Value>),
    Or(Box<NConstraint>, Box<NConstraint>),
    And(Box<NConstraint>, Box<NConstraint>),
    App(Identifier, LinkedList<TypArg>),
    Var(KindIdentifier),
    True,
    False,
}

#[derive(Debug, Clone, FromValue)]
pub struct NConstraint(NConstraintAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct Literal(LiteralAux, L);

/// Type pattern
#[derive(Debug, Clone, FromValue)]
pub enum TypPatAux {
    Wild,
    Var(KindIdentifier),
    App(Identifier, LinkedList<TypPat>),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypPat(TypPatAux, L);

/// Kinded identifier or Int constraint
#[derive(Debug, Clone, FromValue)]
pub enum QuantItemAux {
    /// Optionally kinded identifier
    KindedIdentifier(KindedIdentifier),
    /// Constraint for this type
    Constraint(NConstraint),
    Constant(LinkedList<KindedIdentifier>),
}

/// Pattern
#[derive(Debug, Clone, FromValue)]
pub enum PatternAux {
    /// Literal constant pattern
    Literal(Literal),

    /// Always matches
    Wildcard,

    /// Pattern disjunction
    Or(Box<Pattern>, Box<Pattern>),

    /// Pattern negation
    Not(Box<Pattern>),

    /// Named pattern
    As(Box<Pattern>, Identifier),

    /// Typed pattern
    Type(Typ, Box<Pattern>),

    /// Identifier
    Identifier(Identifier),

    /// Bind pattern to type variable
    Variable(Box<Pattern>, TypPat),

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

/// TODO CHECK THIS ORDERING OF `ANNOT`
#[derive(Debug, Clone, FromValue)]
pub struct Pattern(PatternAux, L);

/// Either a kinded identifier or a nexp constraint for a typquant
#[derive(Debug, Clone, FromValue)]
pub struct QuantItem(QuantItemAux, L);

/// Internal syntax for an optional termination measure for a loop
#[derive(Debug, Clone, FromValue)]
pub enum InternalLoopMeasureAux {
    None,
    Some(Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct InternalLoopMeasure(InternalLoopMeasureAux, L);

/// Expression
#[derive(Debug, Clone, FromValue)]
pub enum ExpressionAux {
    /// Sequential block
    Block(LinkedList<Expression>),

    /// Identifier
    Identifier(Identifier),

    /// Literal constant
    Literal(Literal),

    /// Cast
    Cast(Typ, Box<Expression>),

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
        Box<InternalLoopMeasure>,
        Box<Expression>,
        Box<Expression>,
    ),

    /// For loop
    For(
        Identifier,
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Order,
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
    Record(LinkedList<FieldExpression>),

    /// Functional update of struct
    RecordUpdate(Box<Expression>, LinkedList<FieldExpression>),

    /// Field projection from struct
    Field(Box<Expression>, Identifier),

    /// Pattern matching
    Case(Box<Expression>, LinkedList<PatternMatch>),

    /// Let expression
    Let(LetBind, Box<Expression>),

    /// Imperative assignment
    Assign(Box<LValueExpression>, Box<Expression>),

    /// Value of $nexp$ at run time
    SizeOf(NumericExpression),

    /// Return $(exp 'a)$ from current function
    Return(Box<Expression>),

    /// Halt all current execution
    Exit(Box<Expression>),

    Ref(Identifier),

    Throw(Box<Expression>),

    Try(Box<Expression>, LinkedList<PatternMatch>),

    /// Halt with error message $(exp 'a)$ when not $(exp 'a)$. exp' is optional.
    Assert(Box<Expression>, Box<Expression>),

    /// This is an internal node for compilation that demonstrates the scope of a local mutable variable
    Var(Box<LValueExpression>, Box<Expression>, Box<Expression>),

    /// his is an internal node, used to distinguised some introduced lets during processing from original ones
    InternalPLet(Pattern, Box<Expression>, Box<Expression>),

    /// For internal use to embed into monad definition
    InternalReturn(Box<Expression>),

    /// For internal use in interpreter to wrap pre-evaluated values when returning an action
    InternalValue(Box<Value>),

    Constraint(NConstraint),
}

/// Expression
#[derive(Debug, Clone, FromValue)]
pub struct Expression(ExpressionAux, L);

/// l-value expression
#[derive(Debug, Clone, FromValue)]
pub enum LValueExpressionAux {
    Identifier(Identifier),
    Deref(Expression),
    Memory(Identifier, LinkedList<Expression>),
    Cast(Typ, Identifier),
    /// multiple (non-memory) assignment
    Tuple(LinkedList<LValueExpression>),
    /// vector concatenation L-exp
    VectorConcat(LinkedList<LValueExpression>),
    /// vector element
    Vector(Box<LValueExpression>, Expression),
    /// Subvector
    VectorRange(Box<LValueExpression>, Expression, Expression),
    /// Struct field
    Field(Box<LValueExpression>, Identifier),
}

#[derive(Debug, Clone, FromValue)]
pub struct LValueExpression(LValueExpressionAux, L);

/// Field Expression
#[derive(Debug, Clone, FromValue)]
pub struct FieldExpressionAux(pub Identifier, pub Expression);

/// Field Expression
#[derive(Debug, Clone, FromValue)]
pub struct FieldExpression(FieldExpressionAux, L);

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
    Type(Box<MappingPattern>, Typ),
    As(Box<MappingPattern>, Identifier),
}

#[derive(Debug, Clone, FromValue)]
pub struct MappingPattern(MappingPatternAux, L);

/// Type quantifiers and constraints
#[derive(Debug, Clone, FromValue)]
pub enum TypQuantAux {
    Tq(LinkedList<QuantItem>),
    /// Sugar, omitting quantifier and constraints
    NoForAll,
}

#[derive(Debug, Clone, FromValue)]
pub struct RegisterIdAux(Identifier);

/// Mapping pattern expression
#[derive(Debug, Clone, FromValue)]
pub enum MappingPatternExpressionAux {
    Pattern(MappingPattern),
    When(MappingPattern, Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypQuant(TypQuantAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct RegisterId(RegisterIdAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct MappingPatternExpression(MappingPatternExpressionAux, L);

/// Type scheme
#[derive(Debug, Clone, FromValue)]
pub struct TypeSchemeAux(pub TypQuant, pub Typ);

#[derive(Debug, Clone, FromValue)]
pub enum AliasSpecAux {
    SubReg(RegisterId, Identifier),
    Bit(RegisterId, Expression),
    Slice(RegisterId, Expression, Expression),
    Concat(RegisterId, RegisterId),
}

/// Optional type annotation for functions
#[derive(Debug, Clone, FromValue)]
pub enum TypeAnnotationOptAux {
    None,
    Some(TypQuant, Typ),
}

/// Function clause
#[derive(Debug, Clone, FromValue)]
pub struct FunctionClauseAux(pub Identifier, pub PatternMatch);

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

/// Optional effect annotation for functions
#[derive(Debug, Clone, FromValue)]
pub enum EffectAnnotationOptAux {
    /// No effect annotation
    None,
    Some(Effect),
}

/// Type union constructors
#[derive(Debug, Clone, FromValue)]
pub enum TypeUnionAux {
    Identifier(Typ, Identifier),
}

/// Mapping clause (bidirectional pattern-match)
#[derive(Debug, Clone, FromValue)]
pub enum MappingClauseAux {
    Bidirectional(MappingPatternExpression, MappingPatternExpression),
    Forwards(MappingPatternExpression, Expression),
    Backwards(MappingPatternExpression, Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypeScheme(TypeSchemeAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct AliasSpec(AliasSpecAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct TypeAnnotationOpt(TypeAnnotationOptAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct FunctionClause(FunctionClauseAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct RecursiveAnnotationOpt(RecursiveAnnotationOptAux, L);

#[derive(Debug, Clone, FromValue)]
pub enum EffectOpt {
    Inner(EffectAnnotationOptAux, L),
}

#[derive(Debug, Clone, FromValue)]
pub struct TypeUnion(TypeUnionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct MappingClause(MappingClauseAux, L);

/// Index specification, for bitfields in register types
#[derive(Debug, Clone, FromValue)]
pub enum IndexRangeAux {
    /// Single index
    Single(NumericExpression),
    /// Index range
    Range(NumericExpression, NumericExpression),
    /// Concatenation of index ranges
    Concat(Box<IndexRange>, Box<IndexRange>),
}

#[derive(Debug, Clone, FromValue)]
pub struct IndexRange(IndexRangeAux, L);

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
    Register(Effect, Effect, Typ, Identifier),
    Config(Identifier, Typ, Expression),
    Alias(Identifier, AliasSpec),
    TypeAlias(Typ, Identifier, AliasSpec),
}

/// Function definition
#[derive(Debug, Clone, FromValue)]
pub enum FunctionDefinitionAux {
    Function(
        RecursiveAnnotationOpt,
        TypeAnnotationOpt,
        EffectOpt,
        LinkedList<FunctionClause>,
    ),
}

/// default kinding or typing assumption
#[derive(Debug, Clone, FromValue)]
pub enum DefaultSpecAux {
    Order(Order),
}

/// Function and type union definitions that can be spread across a file. Each one must end in $_$
#[derive(Debug, Clone, FromValue)]
pub enum ScatteredDefinitionAux {
    /// Scattered function definition header
    Function(
        RecursiveAnnotationOpt,
        TypeAnnotationOpt,
        EffectOpt,
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

/// Type definition body
#[derive(Debug, Clone, FromValue)]
pub enum TypeDefinitionAux {
    /// Type abbreviation
    Abbreviation(Identifier, TypQuant, TypArg),
    /// Struct type definition
    Record(Identifier, TypQuant, LinkedList<(Typ, Identifier)>, bool),
    /// Tagged union type definition
    Variant(Identifier, TypQuant, LinkedList<TypeUnion>, bool),
    /// Enumeration type definition
    Enumeration(Identifier, LinkedList<Identifier>, bool),
    /// Register mutable bitfield type definition
    Bitfield(Identifier, Typ, LinkedList<(Identifier, IndexRange)>),
}

/// Mapping definition (bidirectional pattern-match function)
#[derive(Debug, Clone, FromValue)]
pub struct MappingDefinitionAux(
    pub Identifier,
    pub TypeAnnotationOpt,
    pub LinkedList<MappingClause>,
);

/// Optional default value for indexed vectors
///
/// To define a default value for any unspecified positions in a sparse map
#[derive(Debug, Clone, FromValue)]
pub enum OptionalDefaultAux {
    Empty,
    Dec(Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct ValueSpecification(ValueSpecificationAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct DecSpec(DecSpecAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct FunctionDefinition(FunctionDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct DefaultSpec(DefaultSpecAux, L);

#[derive(Debug, Clone, FromValue)]
pub enum Prec {
    Infix,
    InfixLeft,
    InfixRight,
}

#[derive(Debug, Clone, FromValue)]
pub enum LoopMeasure {
    Loop(Loop, Expression),
}

#[derive(Debug, Clone, FromValue)]
pub struct ScatteredDefinition(ScatteredDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct TypeDefinition(TypeDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct MappingDefinition(MappingDefinitionAux, L);

#[derive(Debug, Clone, FromValue)]
pub struct OptionalDefault(OptionalDefaultAux, L);

/// Top-level Sail2 definition
#[derive(Debug, Clone, FromValue)]
pub enum Definition {
    /// Type definition
    Type(ocaml::Value),

    /// Function definition
    Function(ocaml::Value),

    /// Mapping definition
    Mapping(MappingDefinition),

    /// Value definition
    Value(LetBind),

    Spec(ocaml::Value),

    /// Fixity declaration
    Fixity(Prec, ocaml::Value, Identifier),

    /// Operator overload specifications
    Overload(Identifier, LinkedList<Identifier>),

    /// Default type and kind assumptions
    Default(DefaultSpec),

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

    /// Internal mutrec
    Mutual(LinkedList<FunctionDefinition>),

    /// Pragma
    Pragma(OCamlString, OCamlString, L),
}

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

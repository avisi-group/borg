#![allow(missing_docs)]

//! Sail abstract syntax tree corresponding to data structures in `ast.ml`,
//! which itself is generated from `ast.lem` and `sail.ott`.

use {
    crate::{
        num::{BigInt, Num},
        types::{OCamlString, Position},
    },
    common::identifiable::identifiable_fromvalue,
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int},
    serde::{Deserialize, Serialize},
    std::collections::LinkedList,
};

/// Location
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
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

#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Mut {
    Immutable,
    Mutable,
}

#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Bit {
    B0,
    B1,
}

/// Sail AST Value
///
/// **Not to be confused with `ocaml::Value`**
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Value {
    Vector(LinkedList<Value>),
    List(LinkedList<Value>),
    Int(BigInt),
    Real(Num),
    Bool(bool),
    Bit(Bit),
    Tuple(LinkedList<Value>),
    Unit,
    String(OCamlString),
    Ref(OCamlString),
    Ctor(OCamlString, LinkedList<Value>),
    Record(LinkedList<(OCamlString, Value)>),
    AttemptedRead(OCamlString),
}

/// Annotation with generic value (ignored as unit here)
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Annot(pub L, ());

/// Loop kind
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Loop {
    While,
    Until,
}

/// Idenitifer
pub type X = OCamlString;

/// Infix identifier
pub type Xi = OCamlString;

#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
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
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct KindIdentifierAux {
    kind_identifier: X,
}

/// Base kind
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
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

#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum IdentifierAux {
    Identifier(X),
    Operator(X),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct BaseEffect {
    pub inner: BaseEffectAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct KindIdentifier {
    pub inner: KindIdentifierAux,
    pub location: L,
}

/// Base kind
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Kind {
    pub inner: KindAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Identifier {
    pub inner: IdentifierAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct EffectAux {
    pub inner: LinkedList<BaseEffect>,
}

/// Vector order specifications, of kind Order
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum OrderAux {
    Variable(KindIdentifier),
    Increasing,
    Decreasing,
}

/// Optionally kind-annotated identifier
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct KindedIdentifierAux {
    pub kind: Kind,
    pub kind_identifier: KindIdentifier,
}

/// Numeric expression, of kind Int
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum NumericExpressionAux {
    /// Abbreviation identifier
    Id(Identifier),
    /// Variable
    Var(KindIdentifier),
    /// Constant
    Constant(BigInt),
    Application(Identifier, LinkedList<NumericExpression>),
    Times(NumericExpression, NumericExpression),
    Sum(NumericExpression, NumericExpression),
    Minus(NumericExpression, NumericExpression),
    Exponential(NumericExpression),
    /// Unary negation
    Negation(NumericExpression),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct NumericExpression {
    pub inner: Box<NumericExpressionAux>,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Effect {
    pub inner: EffectAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Order {
    pub inner: OrderAux,
    pub location: L,
}

/// Optionally kind-annotated identifier
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct KindedIdentifier {
    pub inner: KindIdentifierAux,
    pub location: L,
}

/// Literal constant
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum LiteralAux {
    Unit,
    Zero,
    One,
    True,
    False,
    /// Natural number constant
    Num(BigInt),
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
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum TypAux {
    InternalUnknown,
    /// Defined type
    Id(Identifier),
    /// Type variable
    Var(KindIdentifier),
    /// Function (first-order only)
    Fn(LinkedList<Typ>, Typ, Effect),
    /// Mapping
    BiDir(Typ, Typ, Effect),
    Tuple(LinkedList<Typ>),
    /// Type constructor application
    Application(Identifier, LinkedList<TypArg>),
    Exist(LinkedList<KindedIdentifier>, NConstraint, Typ),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Typ {
    pub inner: Box<TypAux>,
    pub annotation: L,
}

/// Type constructor arguments of all kinds
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum TypArgAux {
    NExp(NumericExpression),
    Typ(Typ),
    Order(Order),
    Bool(NConstraint),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypArg {
    pub inner: TypArgAux,
    pub location: L,
}

/// Constraint over kind Int
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum NConstraintAux {
    Equal(NumericExpression, NumericExpression),
    BoundedGe(NumericExpression, NumericExpression),
    BoundedGt(NumericExpression, NumericExpression),
    BoundedLe(NumericExpression, NumericExpression),
    BoundedLt(NumericExpression, NumericExpression),
    NotEqual(NumericExpression, NumericExpression),
    Set(KindIdentifier, LinkedList<BigInt>),
    Or(NConstraint, NConstraint),
    And(NConstraint, NConstraint),
    App(Identifier, LinkedList<TypArg>),
    Var(KindIdentifier),
    True,
    False,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct NConstraint {
    pub inner: Box<NConstraintAux>,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Literal {
    pub inner: LiteralAux,
    pub location: L,
}

/// Type pattern
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum TypPatAux {
    Wild,
    Var(KindIdentifier),
    App(Identifier, LinkedList<TypPat>),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypPat {
    pub inner: TypPatAux,
    pub location: L,
}

/// Kinded identifier or Int constraint
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum QuantItemAux {
    /// Optionally kinded identifier
    KindedIdentifier(KindedIdentifier),
    /// Constraint for this type
    Constraint(NConstraint),
    Constant(LinkedList<KindedIdentifier>),
}

/// Pattern
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum PatternAux {
    /// Literal constant pattern
    Literal(Literal),

    /// Always matches
    Wildcard,

    /// Pattern disjunction
    Or(Pattern, Pattern),

    /// Pattern negation
    Not(Pattern),

    /// Named pattern
    As(Pattern, Identifier),

    /// Typed pattern
    Type(Typ, Pattern),

    /// Identifier
    Identifier(Identifier),

    /// Bind pattern to type variable
    Variable(Pattern, TypPat),

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
    Cons(Pattern, Pattern),

    /// String append pattern
    ///
    /// x^^y
    StringAppend(LinkedList<Pattern>),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Pattern {
    pub inner: Box<PatternAux>,
    pub annotation: Annot,
}

/// Either a kinded identifier or a nexp constraint for a typquant
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct QuantItem {
    pub inner: QuantItemAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct InternalLoopMeasure {
    pub inner: Option<Box<Expression>>,
    pub location: L,
}

/// Expression
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum ExpressionAux {
    /// Sequential block
    Block(LinkedList<Expression>),

    /// Identifier
    Identifier(Identifier),

    /// Literal constant
    Literal(Literal),

    /// Cast
    Cast(Typ, Expression),

    /// Function application
    Application(Identifier, LinkedList<Expression>),

    /// Infix function application
    ApplicationInfix(Expression, Identifier, Expression),

    /// Tuple
    Tuple(LinkedList<Expression>),

    /// Conditional
    If(Expression, Expression, Expression),

    Loop(Loop, InternalLoopMeasure, Expression, Expression),

    /// For loop
    For(
        Identifier,
        Expression,
        Expression,
        Expression,
        Order,
        Expression,
    ),

    /// Vector (indexed from 0)
    Vector(LinkedList<Expression>),

    /// Vector access
    VectorAccess(Expression, Expression),

    /// Subvector extraction
    VectorSubrange(Expression, Expression, Expression),

    /// Vector functional update
    VectorUpdate(Expression, Expression, Expression),

    /// Vector subrange update (with vector)
    VectorUpdateSubrange(Expression, Expression, Expression, Expression),

    /// Vector concatenation
    VectorAppend(Expression, Expression),

    /// List
    List(LinkedList<Expression>),

    /// Cons
    Cons(Expression, Expression),

    /// Struct
    Record(LinkedList<FieldExpression>),

    /// Functional update of struct
    RecordUpdate(Expression, LinkedList<FieldExpression>),

    /// Field projection from struct
    Field(Expression, Identifier),

    /// Pattern matching
    Case(Expression, LinkedList<PatternMatch>),

    /// Let expression
    Let(LetBind, Expression),

    /// Imperative assignment
    Assign(LValueExpression, Expression),

    /// Value of $nexp$ at run time
    SizeOf(NumericExpression),

    /// Return $(exp 'a)$ from current function
    Return(Expression),

    /// Halt all current execution
    Exit(Expression),

    Ref(Identifier),

    Throw(Expression),

    Try(Expression, LinkedList<PatternMatch>),

    /// Halt with error message $(exp 'a)$ when not $(exp 'a)$. exp' is
    /// optional.
    Assert(Expression, Expression),

    /// This is an internal node for compilation that demonstrates the scope of
    /// a local mutable variable
    Var(LValueExpression, Expression, Expression),

    /// his is an internal node, used to distinguised some introduced lets
    /// during processing from original ones
    InternalPLet(Pattern, Expression, Expression),

    /// For internal use to embed into monad definition
    InternalReturn(Expression),

    /// For internal use in interpreter to wrap pre-evaluated values when
    /// returning an action
    InternalValue(Value),

    Constraint(NConstraint),
}

/// Expression
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Expression {
    pub inner: Box<ExpressionAux>,
    pub annotation: Annot,
}

/// l-value expression
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
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
    Vector(LValueExpression, Expression),
    /// Subvector
    VectorRange(LValueExpression, Expression, Expression),
    /// Struct field
    Field(LValueExpression, Identifier),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct LValueExpression {
    pub inner: Box<LValueExpressionAux>,
    pub annotation: Annot,
}

/// Field Expression
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct FieldExpressionAux {
    pub identifier: Identifier,
    pub expression: Expression,
}

/// Field Expression
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct FieldExpression {
    pub inner: FieldExpressionAux,
    pub annotation: Annot,
}

/// Pattern match
///
/// `pexp` in Sail source
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum PatternMatchAux {
    Expression(Pattern, Expression),
    When(Pattern, Expression, Expression),
}

/// Pattern match
///
/// `pexp` in Sail source
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct PatternMatch {
    pub inner: PatternMatchAux,
    pub annotation: Annot,
}

/// Value binding
///
/// Implicit type, pattern must be total
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct LetBindAux {
    pub pattern: Pattern,
    pub expression: Expression,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct LetBind {
    pub let_bind: LetBindAux,
    pub annotation: Annot,
}

/// Mapping pattern
///
/// Mostly the same as normal patterns but only constructible parts
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
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
    Cons(MappingPattern, MappingPattern),
    /// String append pattern
    ///
    /// x^^y
    StringAppend(LinkedList<MappingPattern>),
    /// Typed pattern
    Type(MappingPattern, Typ),
    As(MappingPattern, Identifier),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingPattern {
    pub inner: Box<MappingPatternAux>,
    pub annotation: Annot,
}

/// Type quantifiers and constraints
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum TypQuantAux {
    Tq(LinkedList<QuantItem>),
    /// Sugar, omitting quantifier and constraints
    NoForAll,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct RegisterIdAux {
    pub identifier: Identifier,
}

/// Mapping pattern expression
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum MappingPatternExpressionAux {
    Pattern(MappingPattern),
    When(MappingPattern, Expression),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypQuant {
    pub inner: TypQuantAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct RegisterId {
    pub inner: RegisterIdAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingPatternExpression {
    pub inner: MappingPatternExpressionAux,
    pub annotation: Annot,
}

/// Type scheme
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeSchemeAux {
    pub typ_quantifier: TypQuant,
    pub typ: Typ,
}

#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum AliasSpecAux {
    SubReg(RegisterId, Identifier),
    Bit(RegisterId, Expression),
    Slice(RegisterId, Expression, Expression),
    Concat(RegisterId, RegisterId),
}

/// Optional type annotation for functions
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum TypeAnnotationOptAux {
    None,
    Some(TypQuant, Typ),
}

/// Function clause
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionClauseAux {
    pub identifier: Identifier,
    pub pattern_match: PatternMatch,
}

/// Optional recursive annotation for functions
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum RecursiveAnnotationOptAux {
    /// Non-recursive
    NonRecursive,
    /// Recursive
    Recursive,
    /// Recursive with termination measure
    Measure(Pattern, Expression),
}

/// Optional effect annotation for functions
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum EffectAnnotationOptAux {
    /// No effect annotation
    None,
    Some(Effect),
}

/// Type union constructors
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum TypeUnionAux {
    Identifier(Typ, Identifier),
}

/// Mapping clause (bidirectional pattern-match)
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum MappingClauseAux {
    Bidirectional(MappingPatternExpression, MappingPatternExpression),
    Forwards(MappingPatternExpression, Expression),
    Backwards(MappingPatternExpression, Expression),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeScheme {
    pub inner: TypeSchemeAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct AliasSpec {
    pub inner: AliasSpecAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeAnnotationOpt {
    pub inner: TypeAnnotationOptAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionClause {
    pub inner: FunctionClauseAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct RecursiveAnnotationOpt {
    pub inner: RecursiveAnnotationOptAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct EffectOpt {
    pub inner: EffectAnnotationOptAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeUnion {
    pub inner: TypeUnionAux,
    pub location: L,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingClause {
    pub inner: MappingClauseAux,
    pub annotation: Annot,
}

/// Index specification, for bitfields in register types
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum IndexRangeAux {
    /// Single index
    Single(NumericExpression),
    /// Index range
    Range(NumericExpression, NumericExpression),
    /// Concatenation of index ranges
    Concat(IndexRange, IndexRange),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct IndexRange {
    pub inner: Box<IndexRangeAux>,
    pub location: L,
}

/// Value type specification
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct ValueSpecificationAux {
    pub typ_scheme: TypeScheme,
    pub ident: Identifier,
    pub a: LinkedList<(OCamlString, OCamlString)>,
    pub b: bool,
}

/// Register declarations
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum DecSpecAux {
    Register(Effect, Effect, Typ, Identifier),
    Config(Identifier, Typ, Expression),
    Alias(Identifier, AliasSpec),
    TypeAlias(Typ, Identifier, AliasSpec),
}

/// Function definition
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionDefinitionAux {
    pub recursive_annotation: RecursiveAnnotationOpt,
    pub type_annotation: TypeAnnotationOpt,
    pub effect: EffectOpt,
    pub clauses: LinkedList<FunctionClause>,
}

/// default kinding or typing assumption
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum DefaultSpecAux {
    Order(Order),
}

/// Function and type union definitions that can be spread across a file. Each
/// one must end in $_$
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
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
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
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
#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingDefinitionAux {
    pub ident: Identifier,
    pub typ_annotation: TypeAnnotationOpt,
    pub clauses: LinkedList<MappingClause>,
}

/// Optional default value for indexed vectors
///
/// To define a default value for any unspecified positions in a sparse map
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum OptionalDefaultAux {
    Empty,
    Dec(Expression),
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct ValueSpecification {
    pub inner: ValueSpecificationAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct DecSpec {
    pub inner: DecSpecAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionDefinition {
    pub inner: FunctionDefinitionAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct DefaultSpec {
    pub inner: DefaultSpecAux,
    pub location: L,
}

#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Prec {
    Infix,
    InfixLeft,
    InfixRight,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct LoopMeasure {
    pub loop0: Loop,
    pub expression: Expression,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct ScatteredDefinition {
    pub inner: ScatteredDefinitionAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeDefinition {
    pub inner: TypeDefinitionAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingDefinition {
    pub inner: MappingDefinitionAux,
    pub annotation: Annot,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct OptionalDefault {
    pub inner: OptionalDefaultAux,
    pub annotation: Annot,
}

/// Top-level Sail2 definition
#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Definition {
    /// Type definition
    Type(TypeDefinition),

    /// Function definition
    Function(FunctionDefinition),

    /// Mapping definition
    Mapping(MappingDefinition),

    /// Value definition
    Value(LetBind),

    Spec(ValueSpecification),

    /// Fixity declaration
    Fixity(Prec, BigInt, Identifier),

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

#[derive(Debug, Clone, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub enum CommentType {
    Block,
    Line,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]

pub struct Comment {
    pub typ: CommentType,
    pub start_position: Position,
    pub end_position: Position,
    pub contents: OCamlString,
}

#[identifiable_fromvalue]
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Ast {
    pub defs: LinkedList<Definition>,
    pub comments: LinkedList<(OCamlString, LinkedList<Comment>)>,
}

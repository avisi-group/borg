#![allow(missing_docs)]

//! Sail abstract syntax tree corresponding to data structures in `ast.ml`,
//! which itself is generated from `ast.lem` and `sail.ott`.

use {
    crate::{
        num::{BigInt, Num},
        types::{KindIdentifierInner, Position},
        visitor::{Visitor, Walkable},
    },
    common::intern::InternedStringKey,
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue},
    serde::{Deserialize, Serialize},
    std::{collections::LinkedList, fmt::Display},
    strum::IntoStaticStr,
};

/// Location
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum L {
    /// Unknown location
    Unknown,
    /// Unique location
    Unique(Int, Box<L>),
    /// Generated location
    Generated(Box<L>),
    /// Hint
    Hint(InternedStringKey, Box<L>, Box<L>),
    /// Range between two positions
    Range(Position, Position),
    /// Documented location
    Documented(InternedStringKey, Box<L>),
}

impl Display for L {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            L::Range(p0, _) => write!(f, "{}", p0),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum Mut {
    Immutable,
    Mutable,
}

#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum Bit {
    B0,
    B1,
}

/// Sail AST Value
///
/// **Not to be confused with `ocaml::Value`**
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum Value {
    Vector(LinkedList<Value>),
    List(LinkedList<Value>),
    Int(BigInt),
    Real(Num),
    Bool(bool),
    Bit(Bit),
    Tuple(LinkedList<Value>),
    Unit,
    String(InternedStringKey),
    Ref(InternedStringKey),
    Ctor(InternedStringKey, LinkedList<Value>),
    Record(LinkedList<(InternedStringKey, Value)>),
    AttemptedRead(InternedStringKey),
}

impl Walkable for Value {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self {
            Value::Vector(vals) => vals.iter().for_each(|val| visitor.visit_value(val)),
            Value::List(vals) => vals.iter().for_each(|val| visitor.visit_value(val)),
            Value::Int(_) => (),
            Value::Real(_) => (),
            Value::Bool(_) => (),
            Value::Bit(_) => (),
            Value::Tuple(vals) => vals.iter().for_each(|val| visitor.visit_value(val)),
            Value::Unit => (),
            Value::String(_) => (),
            Value::Ref(_) => (),
            Value::Ctor(_, vals) => vals.iter().for_each(|val| visitor.visit_value(val)),
            Value::Record(vals) => vals.iter().for_each(|(_, val)| visitor.visit_value(val)),
            Value::AttemptedRead(_) => (),
        }
    }
}

/// Annotation with generic value (ignored as unit here)
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Annot(pub L);

/// Loop kind
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum Loop {
    While,
    Until,
}

/// Idenitifer
pub type X = InternedStringKey;

/// Infix identifier
pub type Xi = InternedStringKey;

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
struct Extern {
    pure: bool,
    bindings: LinkedList<(InternedStringKey, InternedStringKey)>,
}

/// kinded IDs: Type, Int, and Order variables

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct KindIdentifierAux {
    kind_identifier: KindIdentifierInner,
}

/// Base kind
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum IdentifierAux {
    Identifier(X),
    Operator(X),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct KindIdentifier {
    pub inner: KindIdentifierAux,
    pub location: L,
}

impl Walkable for KindIdentifier {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// Base kind

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Kind {
    pub inner: KindAux,
    pub location: L,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Identifier {
    pub inner: IdentifierAux,
    pub location: L,
}

impl Identifier {
    pub fn get_string(&self) -> InternedStringKey {
        match self.inner {
            IdentifierAux::Identifier(s) => s,
            IdentifierAux::Operator(s) => s,
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Walkable for Identifier {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// Vector order specifications, of kind Order
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum OrderAux {
    Variable(KindIdentifier),
    Increasing,
    Decreasing,
}

/// Optionally kind-annotated identifier

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct KindedIdentifierAux {
    pub kind: Kind,
    pub kind_identifier: KindIdentifier,
}

/// Numeric expression, of kind Int
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct NumericExpression {
    pub inner: Box<NumericExpressionAux>,
    pub location: L,
}

impl Walkable for NumericExpression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            NumericExpressionAux::Id(id) => visitor.visit_identifier(id),
            NumericExpressionAux::Var(kid) => visitor.visit_kind_identifier(kid),
            NumericExpressionAux::Constant(bi) => visitor.visit_big_int(bi),
            NumericExpressionAux::Application(id, nexps) => {
                visitor.visit_identifier(id);
                nexps
                    .iter()
                    .for_each(|nexp| visitor.visit_numeric_expression(nexp))
            }
            NumericExpressionAux::Times(nexp0, nexp1) => {
                visitor.visit_numeric_expression(nexp0);
                visitor.visit_numeric_expression(nexp1);
            }
            NumericExpressionAux::Sum(nexp0, nexp1) => {
                visitor.visit_numeric_expression(nexp0);
                visitor.visit_numeric_expression(nexp1);
            }
            NumericExpressionAux::Minus(nexp0, nexp1) => {
                visitor.visit_numeric_expression(nexp0);
                visitor.visit_numeric_expression(nexp1);
            }
            NumericExpressionAux::Exponential(nexp) => visitor.visit_numeric_expression(nexp),
            NumericExpressionAux::Negation(nexp) => visitor.visit_numeric_expression(nexp),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Order {
    pub inner: OrderAux,
    pub location: L,
}

impl Walkable for Order {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            OrderAux::Variable(kid) => visitor.visit_kind_identifier(kid),
            OrderAux::Increasing | OrderAux::Decreasing => (),
        }
    }
}

/// Optionally kind-annotated identifier

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct KindedIdentifier {
    pub inner: KindIdentifierAux,
    pub location: L,
}

impl Walkable for KindedIdentifier {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// Literal constant
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum LiteralAux {
    Unit,
    Zero,
    One,
    True,
    False,
    /// Natural number constant
    Num(BigInt),
    /// Bit vector constant, C-style
    Hex(InternedStringKey),
    /// Bit vector constant, C-style
    Bin(InternedStringKey),
    /// String constant
    String(InternedStringKey),
    /// Undefined value constant
    Undefined,
    Real(InternedStringKey),
}

/// Type expressions, of kind Type
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum TypAux {
    InternalUnknown,
    /// Defined type
    Id(Identifier),
    /// Type variable
    Var(KindIdentifier),
    /// Function (first-order only)
    Fn(LinkedList<Typ>, Typ),
    /// Mapping
    BiDir(Typ, Typ),
    Tuple(LinkedList<Typ>),
    /// Type constructor application
    Application(Identifier, LinkedList<TypArg>),
    Exist(LinkedList<KindedIdentifier>, NConstraint, Typ),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Typ {
    pub inner: Box<TypAux>,
    pub annotation: L,
}

impl Walkable for Typ {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            TypAux::InternalUnknown => (),
            TypAux::Id(id) => visitor.visit_identifier(id),
            TypAux::Var(kid) => visitor.visit_kind_identifier(kid),
            TypAux::Fn(typs, typ) => {
                typs.iter().for_each(|t| visitor.visit_typ(t));
                visitor.visit_typ(typ);
            }
            TypAux::BiDir(typ0, typ1) => {
                visitor.visit_typ(typ0);
                visitor.visit_typ(typ1);
            }
            TypAux::Tuple(typs) => typs.iter().for_each(|typ| visitor.visit_typ(typ)),
            TypAux::Application(_, typargs) => typargs
                .iter()
                .for_each(|typarg| visitor.visit_typarg(typarg)),
            TypAux::Exist(kids, nconstraint, typ) => {
                kids.iter()
                    .for_each(|kid| visitor.visit_kinded_identifier(kid));
                visitor.visit_nconstraint(nconstraint);
                visitor.visit_typ(typ);
            }
        }
    }
}

/// Type constructor arguments of all kinds
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum TypArgAux {
    NExp(NumericExpression),
    Typ(Typ),
    Order(Order),
    Bool(NConstraint),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypArg {
    pub inner: TypArgAux,
    pub location: L,
}

impl Walkable for TypArg {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            TypArgAux::NExp(n) => visitor.visit_numeric_expression(n),
            TypArgAux::Typ(n) => visitor.visit_typ(n),
            TypArgAux::Order(n) => visitor.visit_order(n),
            TypArgAux::Bool(n) => visitor.visit_nconstraint(n),
        }
    }
}

/// Constraint over kind Int
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct NConstraint {
    pub inner: Box<NConstraintAux>,
    pub location: L,
}

impl Walkable for NConstraint {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            NConstraintAux::Equal(n1, n2) => {
                visitor.visit_numeric_expression(n1);
                visitor.visit_numeric_expression(n2);
            }
            NConstraintAux::BoundedGe(n1, n2) => {
                visitor.visit_numeric_expression(n1);
                visitor.visit_numeric_expression(n2);
            }
            NConstraintAux::BoundedGt(n1, n2) => {
                visitor.visit_numeric_expression(n1);
                visitor.visit_numeric_expression(n2);
            }
            NConstraintAux::BoundedLe(n1, n2) => {
                visitor.visit_numeric_expression(n1);
                visitor.visit_numeric_expression(n2);
            }
            NConstraintAux::BoundedLt(n1, n2) => {
                visitor.visit_numeric_expression(n1);
                visitor.visit_numeric_expression(n2);
            }
            NConstraintAux::NotEqual(n1, n2) => {
                visitor.visit_numeric_expression(n1);
                visitor.visit_numeric_expression(n2);
            }
            NConstraintAux::Set(ki, bigints) => {
                visitor.visit_kind_identifier(ki);
                bigints.iter().for_each(|bi| visitor.visit_big_int(bi));
            }
            NConstraintAux::Or(n1, n2) => {
                visitor.visit_nconstraint(n1);
                visitor.visit_nconstraint(n2);
            }
            NConstraintAux::And(n1, n2) => {
                visitor.visit_nconstraint(n1);
                visitor.visit_nconstraint(n2);
            }
            NConstraintAux::App(ident, typargs) => {
                visitor.visit_identifier(ident);
                typargs.iter().for_each(|ta| visitor.visit_typarg(ta));
            }
            NConstraintAux::Var(ki) => visitor.visit_kind_identifier(ki),
            NConstraintAux::True | NConstraintAux::False => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Literal {
    pub inner: LiteralAux,
    pub location: L,
}

impl Walkable for Literal {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// Type pattern
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum TypPatAux {
    Wild,
    Var(KindIdentifier),
    App(Identifier, LinkedList<TypPat>),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypPat {
    pub inner: TypPatAux,
    pub location: L,
}

impl Walkable for TypPat {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            TypPatAux::Wild => (),
            TypPatAux::Var(kid) => visitor.visit_kind_identifier(kid),
            TypPatAux::App(ident, typpats) => {
                visitor.visit_identifier(ident);
                typpats
                    .iter()
                    .for_each(|typpat| visitor.visit_typpat(typpat));
            }
        }
    }
}

/// Kinded identifier or Int constraint
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum QuantItemAux {
    /// Optionally kinded identifier
    KindedIdentifier(KindedIdentifier),
    /// Constraint for this type
    Constraint(NConstraint),
    Constant(LinkedList<KindedIdentifier>),
}

/// Pattern
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Pattern {
    pub inner: Box<PatternAux>,
    pub annotation: Annot,
}

impl Walkable for Pattern {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            PatternAux::Literal(lit) => visitor.visit_literal(lit),
            PatternAux::Wildcard => (),
            PatternAux::Or(pat0, pat1) => {
                visitor.visit_pattern(pat0);
                visitor.visit_pattern(pat1);
            }
            PatternAux::Not(pat) => visitor.visit_pattern(pat),
            PatternAux::As(pat, id) => {
                visitor.visit_pattern(pat);
                visitor.visit_identifier(id);
            }
            PatternAux::Type(typ, pat) => {
                visitor.visit_typ(typ);
                visitor.visit_pattern(pat);
            }
            PatternAux::Identifier(id) => visitor.visit_identifier(id),
            PatternAux::Variable(pat, typpat) => {
                visitor.visit_pattern(pat);
                visitor.visit_typpat(typpat);
            }
            PatternAux::Application(id, pats) => {
                visitor.visit_identifier(id);
                pats.iter().for_each(|pat| visitor.visit_pattern(pat));
            }
            PatternAux::Vector(pats) => pats.iter().for_each(|pat| visitor.visit_pattern(pat)),
            PatternAux::VectorConcat(pats) => {
                pats.iter().for_each(|pat| visitor.visit_pattern(pat))
            }
            PatternAux::Tuple(pats) => pats.iter().for_each(|pat| visitor.visit_pattern(pat)),
            PatternAux::List(pats) => pats.iter().for_each(|pat| visitor.visit_pattern(pat)),
            PatternAux::Cons(pat0, pat1) => {
                visitor.visit_pattern(pat0);
                visitor.visit_pattern(pat1);
            }
            PatternAux::StringAppend(pats) => {
                pats.iter().for_each(|pat| visitor.visit_pattern(pat))
            }
        }
    }
}

/// Either a kinded identifier or a nexp constraint for a typquant

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct QuantItem {
    pub inner: QuantItemAux,
    pub location: L,
}

impl Walkable for QuantItem {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            QuantItemAux::KindedIdentifier(n) => visitor.visit_kinded_identifier(n),
            QuantItemAux::Constraint(n) => visitor.visit_nconstraint(n),
            QuantItemAux::Constant(ns) => {
                ns.iter().for_each(|n| visitor.visit_kinded_identifier(n))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct InternalLoopMeasure {
    pub inner: Option<Box<Expression>>,
    pub location: L,
}

impl Walkable for InternalLoopMeasure {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        if let Some(exp) = &self.inner {
            visitor.visit_expression(exp)
        }
    }
}

/// Expression
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

    /// Internal node for additional type checker assumptions
    InternalAssume(NConstraint, Expression),

    Constraint(NConstraint),
}

/// Expression

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Expression {
    pub inner: Box<ExpressionAux>,
    pub annotation: Annot,
}

impl Walkable for Expression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            ExpressionAux::Block(exps) => exps.iter().for_each(|exp| visitor.visit_expression(exp)),
            ExpressionAux::Identifier(id) => visitor.visit_identifier(id),
            ExpressionAux::Literal(lit) => visitor.visit_literal(lit),
            ExpressionAux::Cast(typ, exp) => {
                visitor.visit_typ(typ);
                visitor.visit_expression(exp);
            }
            ExpressionAux::Application(id, exps) => {
                visitor.visit_identifier(id);
                exps.iter().for_each(|exp| visitor.visit_expression(exp));
            }
            ExpressionAux::ApplicationInfix(exp0, id, exp1) => {
                visitor.visit_expression(exp0);
                visitor.visit_identifier(id);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::Tuple(exps) => exps.iter().for_each(|exp| visitor.visit_expression(exp)),
            ExpressionAux::If(exp0, exp1, exp2) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
                visitor.visit_expression(exp2);
            }
            ExpressionAux::Loop(_, internal_loop_measure, exp0, exp1) => {
                visitor.visit_internal_loop_measure(internal_loop_measure);
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::For(id, exp0, exp1, exp2, order, exp3) => {
                visitor.visit_identifier(id);
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
                visitor.visit_expression(exp2);
                visitor.visit_order(order);
                visitor.visit_expression(exp3);
            }
            ExpressionAux::Vector(exps) => {
                exps.iter().for_each(|exp| visitor.visit_expression(exp))
            }

            ExpressionAux::VectorAccess(exp0, exp1) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::VectorSubrange(exp0, exp1, exp2) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
                visitor.visit_expression(exp2);
            }
            ExpressionAux::VectorUpdate(exp0, exp1, exp2) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
                visitor.visit_expression(exp2);
            }
            ExpressionAux::VectorUpdateSubrange(exp0, exp1, exp2, exp3) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
                visitor.visit_expression(exp2);
                visitor.visit_expression(exp3);
            }
            ExpressionAux::VectorAppend(exp0, exp1) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::List(exps) => exps.iter().for_each(|exp| visitor.visit_expression(exp)),
            ExpressionAux::Cons(exp0, exp1) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::Record(exps) => exps
                .iter()
                .for_each(|exp| visitor.visit_field_expression(exp)),
            ExpressionAux::RecordUpdate(exp, exps) => {
                visitor.visit_expression(exp);
                exps.iter()
                    .for_each(|exp| visitor.visit_field_expression(exp))
            }
            ExpressionAux::Field(exp, id) => {
                visitor.visit_expression(exp);
                visitor.visit_identifier(id);
            }
            ExpressionAux::Case(exp, pats) => {
                visitor.visit_expression(exp);
                pats.iter().for_each(|pat| visitor.visit_pattern_match(pat));
            }
            ExpressionAux::Let(letbind, exp) => {
                visitor.visit_letbind(letbind);
                visitor.visit_expression(exp);
            }
            ExpressionAux::Assign(lvalexp, exp) => {
                visitor.visit_lvalue_expression(lvalexp);
                visitor.visit_expression(exp);
            }
            ExpressionAux::SizeOf(nexp) => {
                visitor.visit_numeric_expression(nexp);
            }
            ExpressionAux::Return(exp) => visitor.visit_expression(exp),
            ExpressionAux::Exit(exp) => visitor.visit_expression(exp),
            ExpressionAux::Ref(id) => visitor.visit_identifier(id),
            ExpressionAux::Throw(exp) => visitor.visit_expression(exp),
            ExpressionAux::Try(exp, pats) => {
                visitor.visit_expression(exp);
                pats.iter().for_each(|pat| visitor.visit_pattern_match(pat));
            }
            ExpressionAux::Assert(exp0, exp1) => {
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::Var(lvalexp, exp0, exp1) => {
                visitor.visit_lvalue_expression(lvalexp);
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::InternalPLet(pat, exp0, exp1) => {
                visitor.visit_pattern(pat);
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
            ExpressionAux::InternalReturn(exp) => visitor.visit_expression(exp),
            ExpressionAux::InternalValue(val) => visitor.visit_value(val),
            ExpressionAux::InternalAssume(ncon, exp) => {
                visitor.visit_nconstraint(ncon);
                visitor.visit_expression(exp);
            }
            ExpressionAux::Constraint(ncon) => visitor.visit_nconstraint(ncon),
        }
    }
}

/// l-value expression
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct LValueExpression {
    pub inner: Box<LValueExpressionAux>,
    pub annotation: Annot,
}

impl Walkable for LValueExpression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            LValueExpressionAux::Identifier(ident) => visitor.visit_identifier(ident),

            LValueExpressionAux::Deref(exp) => visitor.visit_expression(exp),

            LValueExpressionAux::Memory(ident, exps) => {
                visitor.visit_identifier(ident);
                exps.iter().for_each(|exp| visitor.visit_expression(exp));
            }

            LValueExpressionAux::Cast(typ, ident) => {
                visitor.visit_typ(typ);
                visitor.visit_identifier(ident);
            }

            LValueExpressionAux::Tuple(lvalexps) => lvalexps
                .iter()
                .for_each(|lvalexp| visitor.visit_lvalue_expression(lvalexp)),

            LValueExpressionAux::VectorConcat(lvalexps) => lvalexps
                .iter()
                .for_each(|lvalexp| visitor.visit_lvalue_expression(lvalexp)),

            LValueExpressionAux::Vector(lvalexp, exp) => {
                visitor.visit_lvalue_expression(lvalexp);
                visitor.visit_expression(exp);
            }

            LValueExpressionAux::VectorRange(lvalexp, exp0, exp1) => {
                visitor.visit_lvalue_expression(lvalexp);
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }

            LValueExpressionAux::Field(lvalexp, ident) => {
                visitor.visit_lvalue_expression(lvalexp);
                visitor.visit_identifier(ident);
            }
        }
    }
}

/// Field Expression

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct FieldExpressionAux {
    pub identifier: Identifier,
    pub expression: Expression,
}

/// Field Expression

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct FieldExpression {
    pub inner: FieldExpressionAux,
    pub annotation: Annot,
}

impl Walkable for FieldExpression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_identifier(&self.inner.identifier);
        visitor.visit_expression(&self.inner.expression);
    }
}

/// Pattern match
///
/// `pexp` in Sail source
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum PatternMatchAux {
    Expression(Pattern, Expression),
    When(Pattern, Expression, Expression),
}

/// Pattern match
///
/// `pexp` in Sail source

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct PatternMatch {
    pub inner: PatternMatchAux,
    pub annotation: Annot,
}

impl Walkable for PatternMatch {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            PatternMatchAux::Expression(pat, exp) => {
                visitor.visit_pattern(pat);
                visitor.visit_expression(exp);
            }
            PatternMatchAux::When(pat, exp0, exp1) => {
                visitor.visit_pattern(pat);
                visitor.visit_expression(exp0);
                visitor.visit_expression(exp1);
            }
        }
    }
}

/// Value binding
///
/// Implicit type, pattern must be total

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct LetBindAux {
    pub pattern: Pattern,
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct LetBind {
    pub let_bind: LetBindAux,
    pub annotation: Annot,
}

impl Walkable for LetBind {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_pattern(&self.let_bind.pattern);
        visitor.visit_expression(&self.let_bind.expression);
    }
}

/// Mapping pattern
///
/// Mostly the same as normal patterns but only constructible parts
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingPattern {
    pub inner: Box<MappingPatternAux>,
    pub annotation: Annot,
}

impl Walkable for MappingPattern {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            MappingPatternAux::Literal(lit) => visitor.visit_literal(lit),
            MappingPatternAux::Identifier(ident) => visitor.visit_identifier(ident),
            MappingPatternAux::Application(ident, mappats) => {
                visitor.visit_identifier(ident);
                mappats
                    .iter()
                    .for_each(|mappat| visitor.visit_mapping_pattern(mappat));
            }
            MappingPatternAux::Vector(mappats) => mappats
                .iter()
                .for_each(|mappat| visitor.visit_mapping_pattern(mappat)),
            MappingPatternAux::VectorConcat(mappats) => mappats
                .iter()
                .for_each(|mappat| visitor.visit_mapping_pattern(mappat)),
            MappingPatternAux::Tuple(mappats) => mappats
                .iter()
                .for_each(|mappat| visitor.visit_mapping_pattern(mappat)),
            MappingPatternAux::List(mappats) => mappats
                .iter()
                .for_each(|mappat| visitor.visit_mapping_pattern(mappat)),
            MappingPatternAux::Cons(mappat0, mappat1) => {
                visitor.visit_mapping_pattern(mappat0);
                visitor.visit_mapping_pattern(mappat1);
            }
            MappingPatternAux::StringAppend(mappats) => mappats
                .iter()
                .for_each(|mappat| visitor.visit_mapping_pattern(mappat)),
            MappingPatternAux::Type(mappat, typ) => {
                visitor.visit_mapping_pattern(mappat);
                visitor.visit_typ(typ);
            }
            MappingPatternAux::As(mappat, ident) => {
                visitor.visit_mapping_pattern(mappat);
                visitor.visit_identifier(ident);
            }
        }
    }
}

/// Type quantifiers and constraints
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum TypQuantAux {
    Tq(LinkedList<QuantItem>),
    /// Sugar, omitting quantifier and constraints
    NoForAll,
}

/// Mapping pattern expression
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum MappingPatternExpressionAux {
    Pattern(MappingPattern),
    When(MappingPattern, Expression),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypQuant {
    pub inner: TypQuantAux,
    pub location: L,
}

impl Walkable for TypQuant {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        let TypQuantAux::Tq(tq) = &self.inner else {
            return;
        };

        tq.iter().for_each(|q| visitor.visit_quantitem(q));
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingPatternExpression {
    pub inner: MappingPatternExpressionAux,
    pub annotation: Annot,
}

impl Walkable for MappingPatternExpression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            MappingPatternExpressionAux::Pattern(mappat) => visitor.visit_mapping_pattern(mappat),
            MappingPatternExpressionAux::When(mappat, exp) => {
                visitor.visit_mapping_pattern(mappat);
                visitor.visit_expression(exp);
            }
        }
    }
}

/// Type scheme

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeSchemeAux {
    pub typ_quantifier: TypQuant,
    pub typ: Typ,
}

/// Optional type annotation for functions
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum TypeAnnotationOptAux {
    None,
    Some(TypQuant, Typ),
}

/// Function clause

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionClauseAux {
    pub identifier: Identifier,
    pub pattern_match: PatternMatch,
}

/// Mapping clause (bidirectional pattern-match)
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum MappingClauseAux {
    Bidirectional(MappingPatternExpression, MappingPatternExpression),
    Forwards(MappingPatternExpression, Expression),
    Backwards(MappingPatternExpression, Expression),
}

/// Type union constructors
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum TypeUnionAux {
    Identifier(Typ, Identifier),
}

/// Optional recursive annotation for functions
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum RecursiveAnnotationOptAux {
    /// Non-recursive
    NonRecursive,
    /// Recursive
    Recursive,
    /// Recursive with termination measure
    Measure(Pattern, Expression),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeScheme {
    pub inner: TypeSchemeAux,
    pub location: L,
}

impl Walkable for TypeScheme {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_typquant(&self.inner.typ_quantifier);
        visitor.visit_typ(&self.inner.typ);
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeAnnotationOpt {
    pub inner: TypeAnnotationOptAux,
    pub location: L,
}

impl Walkable for TypeAnnotationOpt {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            TypeAnnotationOptAux::None => (),
            TypeAnnotationOptAux::Some(typquant, typ) => {
                visitor.visit_typquant(typquant);
                visitor.visit_typ(typ);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionClause {
    pub inner: FunctionClauseAux,
    pub annotation: Annot,
}

impl Walkable for FunctionClause {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_identifier(&self.inner.identifier);
        visitor.visit_pattern_match(&self.inner.pattern_match);
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct RecursiveAnnotationOpt {
    pub inner: RecursiveAnnotationOptAux,
    pub location: L,
}

impl Walkable for RecursiveAnnotationOpt {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            RecursiveAnnotationOptAux::NonRecursive | RecursiveAnnotationOptAux::Recursive => (),
            RecursiveAnnotationOptAux::Measure(pat, exp) => {
                visitor.visit_pattern(pat);
                visitor.visit_expression(exp);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeUnion {
    pub inner: TypeUnionAux,
    pub location: L,
}

impl Walkable for TypeUnion {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            TypeUnionAux::Identifier(typ, ident) => {
                visitor.visit_typ(typ);
                visitor.visit_identifier(ident);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingClause {
    pub inner: MappingClauseAux,
    pub annotation: Annot,
}

impl Walkable for MappingClause {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            MappingClauseAux::Bidirectional(mappatexp0, mappatexp1) => {
                visitor.visit_mapping_pattern_expression(mappatexp0);
                visitor.visit_mapping_pattern_expression(mappatexp1);
            }
            MappingClauseAux::Forwards(mappatexp, exp) => {
                visitor.visit_mapping_pattern_expression(mappatexp);
                visitor.visit_expression(exp);
            }
            MappingClauseAux::Backwards(mappatexp, exp) => {
                visitor.visit_mapping_pattern_expression(mappatexp);
                visitor.visit_expression(exp);
            }
        }
    }
}

/// Index specification, for bitfields in register types
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum IndexRangeAux {
    /// Single index
    Single(NumericExpression),
    /// Index range
    Range(NumericExpression, NumericExpression),
    /// Concatenation of index ranges
    Concat(IndexRange, IndexRange),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct IndexRange {
    pub inner: Box<IndexRangeAux>,
    pub location: L,
}

impl Walkable for IndexRange {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &*self.inner {
            IndexRangeAux::Single(nexp) => visitor.visit_numeric_expression(nexp),
            IndexRangeAux::Range(nexp0, nexp1) => {
                visitor.visit_numeric_expression(nexp0);
                visitor.visit_numeric_expression(nexp1);
            }
            IndexRangeAux::Concat(range0, range1) => {
                visitor.visit_indexrange(range0);
                visitor.visit_indexrange(range1);
            }
        }
    }
}

/// Function and type union definitions that can be spread across a file. Each
/// one must end in $_$
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum ScatteredDefinitionAux {
    /// Scattered function definition header
    Function(RecursiveAnnotationOpt, TypeAnnotationOpt, Identifier),
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

#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum InstantiationSpecificationAux {
    Id(Identifier),
}

/// Type definition body
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
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

/// Register declarations
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum DecSpecAux {
    Register(Typ, Identifier, Option<Expression>),
}

/// Instantiation substitution
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum SubstitutionAux {
    /// Instantiate a type variable with a type
    Typ(KindIdentifier, Typ),
    /// Instantiate an identifier with another identifier
    Id(Identifier, Identifier),
}

/// Mapping definition (bidirectional pattern-match function)

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingDefinitionAux {
    pub ident: Identifier,
    pub typ_annotation: TypeAnnotationOpt,
    pub clauses: LinkedList<MappingClause>,
}

/// Value type specification

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct ValueSpecificationAux {
    pub typ_scheme: TypeScheme,
    pub ident: Identifier,
    pub a: (),
    pub b: bool,
}

/// Outcome declaration
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum OutcomeSpecificationAux {
    Outcome(Identifier, TypeScheme, LinkedList<KindedIdentifier>),
}

/// Default kinding or typing assumption
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum DefaultSpecAux {
    Order(Order),
}

/// Function definition

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionDefinitionAux {
    pub recursive_annotation: RecursiveAnnotationOpt,
    pub type_annotation: TypeAnnotationOpt,
    pub clauses: LinkedList<FunctionClause>,
}

/// Optional default value for indexed vectors
///
/// To define a default value for any unspecified positions in a sparse map
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum OptionalDefaultAux {
    Empty,
    Dec(Expression),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct ScatteredDefinition {
    pub inner: ScatteredDefinitionAux,
    pub annotation: Annot,
}

impl Walkable for ScatteredDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            ScatteredDefinitionAux::Function(recannotopt, typeannotopt, ident) => {
                visitor.visit_recursive_annotation_opt(recannotopt);
                visitor.visit_type_annotation_opt(typeannotopt);

                visitor.visit_identifier(ident);
            }
            ScatteredDefinitionAux::FunctionClause(funcl) => visitor.visit_function_clause(funcl),
            ScatteredDefinitionAux::Variant(ident, typquant) => {
                visitor.visit_identifier(ident);
                visitor.visit_typquant(typquant);
            }
            ScatteredDefinitionAux::UnionClause(ident, typeunion) => {
                visitor.visit_identifier(ident);
                visitor.visit_typunion(typeunion);
            }
            ScatteredDefinitionAux::Mapping(ident, typeannotopt) => {
                visitor.visit_identifier(ident);
                visitor.visit_type_annotation_opt(typeannotopt);
            }
            ScatteredDefinitionAux::MappingClause(ident, mapcl) => {
                visitor.visit_identifier(ident);
                visitor.visit_mapping_clause(mapcl);
            }
            ScatteredDefinitionAux::End(ident) => visitor.visit_identifier(ident),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct InstantiationSpecification {
    pub inner: InstantiationSpecificationAux,
    pub annotation: Annot,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct TypeDefinition {
    pub inner: TypeDefinitionAux,
    pub annotation: Annot,
}

impl Walkable for TypeDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            TypeDefinitionAux::Abbreviation(ident, typquant, typarg) => {
                visitor.visit_identifier(ident);
                visitor.visit_typquant(typquant);
                visitor.visit_typarg(typarg);
            }
            TypeDefinitionAux::Record(ident, typquant, typidents, _) => {
                visitor.visit_identifier(ident);
                visitor.visit_typquant(typquant);
                typidents.iter().for_each(|(typ, ident)| {
                    visitor.visit_typ(typ);
                    visitor.visit_identifier(ident);
                });
            }
            TypeDefinitionAux::Variant(ident, typquant, typunions, _) => {
                visitor.visit_identifier(ident);
                visitor.visit_typquant(typquant);
                typunions
                    .iter()
                    .for_each(|typunion| visitor.visit_typunion(typunion));
            }
            TypeDefinitionAux::Enumeration(ident, idents, _) => {
                visitor.visit_identifier(ident);
                idents
                    .iter()
                    .for_each(|ident| visitor.visit_identifier(ident));
            }
            TypeDefinitionAux::Bitfield(ident, typ, identranges) => {
                visitor.visit_identifier(ident);
                visitor.visit_typ(typ);
                identranges.iter().for_each(|(ident, range)| {
                    visitor.visit_identifier(ident);
                    visitor.visit_indexrange(range);
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct DecSpec {
    pub inner: DecSpecAux,
    pub annotation: Annot,
}

impl Walkable for DecSpec {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            DecSpecAux::Register(typ, ident, exp) => {
                visitor.visit_typ(typ);
                visitor.visit_identifier(ident);
                match exp {
                    Some(exp) => visitor.visit_expression(exp),
                    None => (),
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Substitution {
    pub inner: SubstitutionAux,
    pub location: L,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct MappingDefinition {
    pub inner: MappingDefinitionAux,
    pub annotation: Annot,
}

impl Walkable for MappingDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_identifier(&self.inner.ident);
        visitor.visit_type_annotation_opt(&self.inner.typ_annotation);
        self.inner
            .clauses
            .iter()
            .for_each(|mapcl| visitor.visit_mapping_clause(mapcl));
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct ValueSpecification {
    pub inner: ValueSpecificationAux,
    pub annotation: Annot,
}

impl Walkable for ValueSpecification {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_type_scheme(&self.inner.typ_scheme);
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct OutcomeSpecification {
    pub inner: OutcomeSpecificationAux,
    pub location: L,
}

#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum Prec {
    Infix,
    InfixLeft,
    InfixRight,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct DefaultSpec {
    pub inner: DefaultSpecAux,
    pub location: L,
}

impl Walkable for DefaultSpec {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            DefaultSpecAux::Order(order) => visitor.visit_order(order),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct LoopMeasure {
    pub loop0: Loop,
    pub expression: Expression,
}

impl Walkable for LoopMeasure {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_expression(&self.expression);
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct FunctionDefinition {
    pub inner: FunctionDefinitionAux,
    pub annotation: Annot,
}

impl Walkable for FunctionDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_recursive_annotation_opt(&self.inner.recursive_annotation);
        visitor.visit_type_annotation_opt(&self.inner.type_annotation);
        self.inner
            .clauses
            .iter()
            .for_each(|c| visitor.visit_function_clause(c));
    }
}

#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum ImplDefintionAux {
    Impl(FunctionClause),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct OptionalDefault {
    pub inner: OptionalDefaultAux,
    pub annotation: Annot,
}

/// Top-level Sail2 definition
#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum Definition {
    /// Type definition
    Type(TypeDefinition),

    /// Function definition
    Function(FunctionDefinition),

    /// Mapping definition
    Mapping(MappingDefinition),

    /// Impl definition
    Impl(FunctionClause),

    /// Value definition
    Value(LetBind),

    /// Top-level type constraint
    Spec(ValueSpecification),

    /// Top-level outcome definition
    Outcome(OutcomeSpecification, LinkedList<Definition>),

    /// Instantiation
    Instantiation(InstantiationSpecification, LinkedList<Substitution>),

    /// Fixity declaration
    Fixity(Prec, BigInt, Identifier),

    /// Operator overload specifications
    Overload(Identifier, LinkedList<Identifier>),

    /// Default type and kind assumptions
    Default(DefaultSpec),

    /// Scattered definition
    Scattered(ScatteredDefinition),

    /// Separate termination measure declaration
    Measure {
        identifier: Identifier,
        pattern: Pattern,
        expression: Expression,
    },

    /// Separate termination measure declaration
    LoopMeasures(Identifier, LinkedList<LoopMeasure>),

    /// Register declaration
    Register(DecSpec),

    /// Internal mutrec
    Mutual(LinkedList<FunctionDefinition>),

    /// Pragma
    Pragma(InternedStringKey, InternedStringKey, L),
}

impl Walkable for Definition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self {
            Definition::Type(typedef) => visitor.visit_type_definition(typedef),
            Definition::Function(funcdef) => visitor.visit_function_definition(funcdef),
            Definition::Mapping(mapdef) => visitor.visit_mapping_definition(mapdef),
            Definition::Impl(_) => todo!(),
            Definition::Value(letbind) => visitor.visit_letbind(letbind),
            Definition::Spec(valuespec) => visitor.visit_value_specification(valuespec),
            Definition::Outcome(_, _) => todo!(),
            Definition::Instantiation(_, _) => todo!(),
            Definition::Fixity(_, _, ident) => visitor.visit_identifier(ident),
            Definition::Overload(id, ids) => {
                visitor.visit_identifier(id);
                ids.iter().for_each(|id| visitor.visit_identifier(id));
            }
            Definition::Default(default) => visitor.visit_default_spec(default),
            Definition::Scattered(scattereddef) => visitor.visit_scattered_definition(scattereddef),
            Definition::Measure {
                identifier,
                pattern,
                expression,
            } => {
                visitor.visit_identifier(identifier);
                visitor.visit_pattern(pattern);
                visitor.visit_expression(expression);
            }
            Definition::LoopMeasures(ident, loopmeasures) => {
                visitor.visit_identifier(ident);
                loopmeasures
                    .iter()
                    .for_each(|loopmeasure| visitor.visit_loop_measure(loopmeasure));
            }
            Definition::Register(decspec) => visitor.visit_decspec(decspec),
            Definition::Mutual(funcdefs) => funcdefs
                .iter()
                .for_each(|f| visitor.visit_function_definition(f)),
            Definition::Pragma(_, _, _) => (),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf, IntoStaticStr,
)]
pub enum CommentType {
    Block,
    Line,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct CommentRoot {
    pub path: InternedStringKey,
    pub comments: LinkedList<Comment>,
}

impl Walkable for CommentRoot {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        self.comments.iter().for_each(|c| visitor.visit_comment(c))
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]

pub struct Comment {
    pub typ: CommentType,
    pub start_position: Position,
    pub end_position: Position,
    pub contents: InternedStringKey,
}

impl Walkable for Comment {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Ast {
    pub defs: LinkedList<Definition>,
    pub comments: LinkedList<CommentRoot>,
}

impl Walkable for Ast {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        self.defs.iter().for_each(|d| visitor.visit_definition(d));
        self.comments
            .iter()
            .for_each(|c| visitor.visit_comment_root(c));
    }
}

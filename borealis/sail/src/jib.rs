//! JIB AST
//!
//! JIB abstract syntax tree corresponding to data structures in `jib.ml`,
//! which itself is generated from `jib.lem` and `jib.ott`.

use {
    crate::{
        ast::{Identifier, KindIdentifier, Location},
        num::BigInt,
    },
    common::intern::InternedStringKey,
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue},
    serde::{Deserialize, Serialize},
    std::collections::LinkedList,
};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum BitU {
    B0,
    B1,
    BU,
}

/// Value2.vl
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Vl {
    Bits(LinkedList<BitU>, bool),
    Bit(BitU),
    Bool(bool),
    Unit,
    Int(BigInt),
    String(InternedStringKey),
    Real(InternedStringKey),
    EmptyList,
    Enum(InternedStringKey),
    Ref(InternedStringKey),
    Undefined,
}

/// Name
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Name {
    Name(Identifier, Int),
    Global(Identifier, Int),
    HaveException(Int),
    CurrentException(Int),
    ThrowLocation(Int),
    Return(Int),
}

/// C type
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Ctyp {
    Lint,
    Fint(Int),
    Constant(BigInt),
    Lbits(bool),
    Sbits(Int, bool),
    Fbits(Int, bool),
    Unit,
    Bool,
    Bit,
    String,
    Real,
    Float(Int),
    RoundingMode,
    Tup(LinkedList<Self>),
    Enum(Identifier, LinkedList<Identifier>),
    Struct(
        Identifier,
        LinkedList<((Identifier, LinkedList<Self>), Box<Self>)>,
    ),
    Variant(
        Identifier,
        LinkedList<((Identifier, LinkedList<Self>), Box<Self>)>,
    ),
    Fvector(Int, bool, Box<Self>),
    Vector(bool, Box<Self>),
    List(Box<Self>),
    Ref(Box<Self>),
    Poly(KindIdentifier),
}

/// Operation
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Op {
    Bnot,
    Bor,
    Band,
    ListHead,
    ListTail,
    Eq,
    Neq,
    Ilt,
    Ilteq,
    Igt,
    Igteq,
    Iadd,
    Isub,
    Unsigned(Int),
    Signed(Int),
    Bvnot,
    Bvor,
    Bvand,
    Bvxor,
    Bvadd,
    Bvsub,
    Bvaccess,
    Concat,
    ZeroExtend(Int),
    SignExtend(Int),
    Slice(Int),
    Sslice(Int),
    SetSlice,
    Replicate(Int),
}

/// clexp?
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum ClExp {
    Id(Name, Ctyp),
    Rmw(Name, Name, Ctyp),
    Field(Box<Self>, (Identifier, LinkedList<Ctyp>)),
    Addr(Box<Self>),
    Tuple(Box<Self>, Int),
    Void,
}

/// C value
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum CVal {
    Id(Name, Ctyp),
    Lit(Vl, Ctyp),
    Tuple(LinkedList<Self>, Ctyp),
    Struct(LinkedList<((Identifier, LinkedList<Ctyp>), Self)>, Ctyp),
    CtorKind(Box<Self>, Identifier, LinkedList<Ctyp>, Ctyp),
    CtorUnwrap(Box<Self>, (Identifier, LinkedList<Ctyp>), Ctyp),
    TupleMember(Box<Self>, Int, Int),
    Call(Op, LinkedList<Self>),
    Field(Box<Self>, (Identifier, LinkedList<Ctyp>)),
}

/// Instruction annotation?
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Iannot {
    pub i: Int,
    pub l: Location,
}

/// C type definition
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum CTypeDefinition {
    Enum(Identifier, LinkedList<Identifier>),
    Struct(
        Identifier,
        LinkedList<((Identifier, LinkedList<Ctyp>), Ctyp)>,
    ),
    Variant(
        Identifier,
        LinkedList<((Identifier, LinkedList<Ctyp>), Ctyp)>,
    ),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum InstrAux {
    Decl(Ctyp, Name),
    Init(Ctyp, Name, CVal),
    Jump(CVal, InternedStringKey),
    Goto(InternedStringKey),
    Label(InternedStringKey),
    Funcall(
        ClExp,
        bool,
        (Identifier, LinkedList<Ctyp>),
        LinkedList<CVal>,
    ),
    Copy(ClExp, CVal),
    Clear(Ctyp, Name),
    Undefined(Ctyp),
    Exit(InternedStringKey),
    End(Name),
    If(CVal, LinkedList<Instruction>, LinkedList<Instruction>, Ctyp),
    Block(LinkedList<Instruction>),
    TryBlock(LinkedList<Instruction>),
    Throw(CVal),
    Comment(InternedStringKey),
    Raw(InternedStringKey),
    Return(CVal),
    Reset(Ctyp, Name),
    Reinit(Ctyp, Name, CVal),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Instruction {
    pub inner: InstrAux,
    pub annot: Iannot,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum CDef {
    RegDec(Identifier, Ctyp, LinkedList<Instruction>),
    Type(CTypeDefinition),
    Let(Int, LinkedList<(Identifier, Ctyp)>, LinkedList<Instruction>),
    Spec(
        Identifier,
        Option<InternedStringKey>,
        LinkedList<Ctyp>,
        Ctyp,
    ),
    Fundef(
        Identifier,
        Option<Identifier>,
        LinkedList<Identifier>,
        LinkedList<Instruction>,
    ),
    Startup(Identifier, LinkedList<Instruction>),
    Finish(Identifier, LinkedList<Instruction>),
    Pragma(InternedStringKey, InternedStringKey),
}

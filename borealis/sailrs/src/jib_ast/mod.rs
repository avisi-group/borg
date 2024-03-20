#![allow(missing_docs)]

//! JIB AST
//!
//! JIB abstract syntax tree corresponding to data structures in `jib.ml`,
//! which itself is generated from `jib.lem` and `jib.ott`.

use {
    crate::{
        jib_ast::visitor::{Visitor, Walkable},
        num::BigInt,
        sail_ast::{Identifier, KindIdentifier, Location},
        types::ListVec,
    },
    common::intern::InternedString,
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue},
};

pub mod pretty_print;
pub mod visitor;

/// C type
#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace, __S: rkyv::ser::Serializer"))]

pub enum Type {
    Lint,
    Fint(Int),
    Constant(BigInt),
    Lbits,
    Sbits(Int),
    Fbits(Int),
    Unit,
    Bool,
    Bit,
    String,
    Real,
    Float(Int),
    RoundingMode,
    Tup(#[omit_bounds] ListVec<Self>),
    Enum(Identifier, ListVec<Identifier>),
    Struct(Identifier, #[omit_bounds] ListVec<(Identifier, Self)>),
    Variant(Identifier, #[omit_bounds] ListVec<(Identifier, Self)>),
    Fvector(Int, #[omit_bounds] Box<Self>),
    Vector(#[omit_bounds] Box<Self>),
    List(#[omit_bounds] Box<Self>),
    Ref(#[omit_bounds] Box<Self>),
    Poly(KindIdentifier),
}

impl Walkable for Type {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Lint => (),
            Self::Fint(_) => (),
            Self::Constant(_) => (),
            Self::Lbits => (),
            Self::Sbits(_) => (),
            Self::Fbits(_) => (),
            Self::Unit => (),
            Self::Bool => (),
            Self::Bit => (),
            Self::String => (),
            Self::Real => (),
            Self::Float(_) => (),
            Self::RoundingMode => (),
            Self::Tup(types) => types.iter().for_each(|t| visitor.visit_type(t)),
            Self::Enum(_, _) => (),
            Self::Struct(_, fields) | Self::Variant(_, fields) => {
                fields.iter().for_each(|(_, typ)| {
                    visitor.visit_type(typ);
                });
            }
            Self::Fvector(_, typ) | Self::Vector(typ) | Self::List(typ) | Self::Ref(typ) => {
                visitor.visit_type(typ)
            }
            Self::Poly(_) => (),
        }
    }
}

/// Name
#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub enum Name {
    Name(Identifier, Int),
    Global(Identifier, Int),
    HaveException(Int),
    CurrentException(Int),
    ThrowLocation(Int),
    Return(Int),
}

impl Walkable for Name {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// Operation

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub enum Op {
    Bnot,
    Bor,
    Band,
    ListHead,
    ListTail,
    ListIsEmpty,
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

impl Walkable for Op {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// C value

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace, __S: rkyv::ser::Serializer"))]
pub enum Value {
    Id(Name, Type),
    Lit(Vl, Type),
    Tuple(#[omit_bounds] ListVec<Self>, Type),
    Struct(#[omit_bounds] ListVec<(Identifier, Self)>, Type),
    CtorKind(#[omit_bounds] Box<Self>, (Identifier, ListVec<Type>), Type),
    CtorUnwrap(#[omit_bounds] Box<Self>, (Identifier, ListVec<Type>), Type),
    TupleMember(#[omit_bounds] Box<Self>, Int, Int),
    Call(Op, #[omit_bounds] ListVec<Self>),
    Field(#[omit_bounds] Box<Self>, Identifier),
}

impl Walkable for Value {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Id(name, typ) => {
                visitor.visit_name(name);
                visitor.visit_type(typ);
            }
            Self::Lit(vl, typ) => {
                visitor.visit_vl(vl);
                visitor.visit_type(typ);
            }
            Self::Tuple(values, typ) => {
                values.iter().for_each(|value| visitor.visit_value(value));
                visitor.visit_type(typ);
            }
            Self::Struct(fields, typ) => {
                fields.iter().for_each(|(_, value)| {
                    visitor.visit_value(value);
                });
                visitor.visit_type(typ);
            }
            Self::CtorKind(value, (_, types), typ) => {
                visitor.visit_value(value);
                types.iter().for_each(|typ| visitor.visit_type(typ));
                visitor.visit_type(typ)
            }
            Self::CtorUnwrap(value, (_, types), typ) => {
                visitor.visit_value(value);
                types.iter().for_each(|typ| visitor.visit_type(typ));
                visitor.visit_type(typ)
            }
            Self::TupleMember(value, _, _) => visitor.visit_value(value),
            Self::Call(op, values) => {
                visitor.visit_op(op);
                values.iter().for_each(|value| visitor.visit_value(value));
            }
            Self::Field(value, _) => {
                visitor.visit_value(value);
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub enum BitU {
    B0,
    B1,
    BU,
}

/// Value2.vl

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub enum Vl {
    Bits(ListVec<BitU>),
    Bit(BitU),
    Bool(bool),
    Unit,
    Int(BigInt),
    String(InternedString),
    Real(InternedString),
    Enum(InternedString),
    Ref(InternedString),
    Undefined,
}

impl Walkable for Vl {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// clexp?

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace, __S: rkyv::ser::Serializer"))]
pub enum Expression {
    Id(Name, Type),
    Rmw(Name, Name, Type),
    Field(#[omit_bounds] Box<Self>, Identifier),
    Addr(#[omit_bounds] Box<Self>),
    Tuple(#[omit_bounds] Box<Self>, Int),
    Void,
}

impl Walkable for Expression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Id(name, typ) => {
                visitor.visit_name(name);
                visitor.visit_type(typ);
            }
            Self::Rmw(name0, name1, typ) => {
                visitor.visit_name(name0);
                visitor.visit_name(name1);
                visitor.visit_type(typ);
            }
            Self::Field(expression, _) => {
                visitor.visit_expression(expression);
            }
            Self::Addr(expression) => visitor.visit_expression(expression),
            Self::Tuple(expression, _) => visitor.visit_expression(expression),
            Self::Void => (),
        }
    }
}

type InstructionAnnotation = (Int, Location);

/// C type definition

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub enum TypeDefinition {
    Enum(Identifier, ListVec<Identifier>),
    Struct(Identifier, ListVec<(Identifier, Type)>),
    Variant(Identifier, ListVec<(Identifier, Type)>),
}

impl Walkable for TypeDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Enum(_, _) => (),
            Self::Struct(_, fields) | Self::Variant(_, fields) => {
                fields.iter().for_each(|(_, typ)| {
                    visitor.visit_type(typ);
                });
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace, __S: rkyv::ser::Serializer"))]
pub enum InstructionAux {
    Decl(Type, Name),
    Init(Type, Name, Value),
    Jump(Value, InternedString),
    Goto(InternedString),
    Label(InternedString),
    Funcall(
        Expression,
        bool,
        (Identifier, ListVec<Type>),
        ListVec<Value>,
    ),
    Copy(Expression, Value),
    Clear(Type, Name),
    Undefined(Type),
    Exit(InternedString),
    End(Name),
    If(
        Value,
        #[omit_bounds] ListVec<Instruction>,
        #[omit_bounds] ListVec<Instruction>,
        Type,
    ),
    Block(#[omit_bounds] ListVec<Instruction>),
    TryBlock(#[omit_bounds] ListVec<Instruction>),
    Throw(Value),
    Comment(InternedString),
    Raw(InternedString),
    Return(Value),
    Reset(Type, Name),
    Reinit(Type, Name, Value),
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub struct Instruction {
    pub inner: InstructionAux,
    pub annot: InstructionAnnotation,
}

impl Walkable for Instruction {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            InstructionAux::Decl(typ, name) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
            }
            InstructionAux::Init(typ, name, value) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
                visitor.visit_value(value);
            }
            InstructionAux::Jump(value, _) => visitor.visit_value(value),
            InstructionAux::Goto(_) => {}
            InstructionAux::Label(_) => {}
            InstructionAux::Funcall(expression, _, (_, parameter_types), parameters) => {
                visitor.visit_expression(expression);
                parameter_types
                    .iter()
                    .for_each(|typ| visitor.visit_type(typ));
                parameters
                    .iter()
                    .for_each(|value| visitor.visit_value(value));
            }
            InstructionAux::Copy(expression, value) => {
                visitor.visit_expression(expression);
                visitor.visit_value(value);
            }
            InstructionAux::Clear(typ, name) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
            }
            InstructionAux::Undefined(typ) => visitor.visit_type(typ),
            InstructionAux::Exit(_) => {}
            InstructionAux::End(name) => visitor.visit_name(name),
            InstructionAux::If(value, if_body, else_body, typ) => {
                visitor.visit_value(value);
                if_body.iter().for_each(|i| visitor.visit_instruction(i));
                else_body.iter().for_each(|i| visitor.visit_instruction(i));
                visitor.visit_type(typ);
            }
            InstructionAux::Block(instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            InstructionAux::TryBlock(instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            InstructionAux::Throw(value) => visitor.visit_value(value),
            InstructionAux::Comment(_) => {}
            InstructionAux::Raw(_) => {}
            InstructionAux::Return(value) => visitor.visit_value(value),
            InstructionAux::Reset(typ, name) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
            }
            InstructionAux::Reinit(typ, name, value) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
                visitor.visit_value(value);
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    FromValue,
    ToValue,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    DeepSizeOf,
)]
pub enum Definition {
    Register(Identifier, Type, ListVec<Instruction>),
    Type(TypeDefinition),
    Let(Int, ListVec<(Identifier, Type)>, ListVec<Instruction>),
    Val(Identifier, Option<InternedString>, ListVec<Type>, Type),
    Fundef(
        Identifier,
        Option<Identifier>,
        ListVec<Identifier>,
        ListVec<Instruction>,
    ),
    Startup(Identifier, ListVec<Instruction>),
    Finish(Identifier, ListVec<Instruction>),
    Pragma(InternedString, InternedString),
}

impl Walkable for Definition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Register(_, typ, instructions) => {
                visitor.visit_type(typ);
                instructions
                    .iter()
                    .for_each(|i| visitor.visit_instruction(i));
            }
            Self::Type(type_definition) => visitor.visit_type_definition(type_definition),
            Self::Let(_, types, instructions) => {
                types.iter().for_each(|(_, typ)| visitor.visit_type(typ));
                instructions
                    .iter()
                    .for_each(|i| visitor.visit_instruction(i));
            }
            Self::Val(_, _, types, typ) => {
                types.iter().for_each(|typ| visitor.visit_type(typ));
                visitor.visit_type(typ)
            }
            Self::Fundef(_, _, _, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            Self::Startup(_, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            Self::Finish(_, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            Self::Pragma(_, _) => (),
        }
    }
}

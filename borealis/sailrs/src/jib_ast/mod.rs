#![allow(missing_docs)]

//! JIB AST
//!
//! JIB abstract syntax tree corresponding to data structures in `jib.ml`,
//! which itself is generated from `jib.lem` and `jib.ott`.
//!
//! Do *not* just read https://ocaml.org/p/libsail/latest/doc/Libsail/Jib/index.html, I have lost hours of debugging because of minor differences between the generated JIB file and the rendered docs.

use {
    crate::{
        jib_ast::visitor::{Visitor, Walkable},
        num::BigInt,
        sail_ast::{DefinitionAnnotation, Identifier, KindIdentifier, Location},
        types::ListVec,
    },
    common::intern::InternedString,
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue},
    rkyv::ser::Writer,
};

pub mod pretty_print;
pub mod visitor;

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
pub enum Channel {
    Stdout,
    Stderr,
}

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
#[rkyv(deserialize_bounds(<__D as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source))]
#[rkyv(serialize_bounds(__S: Writer+ rkyv::ser::Allocator))]
#[rkyv(bytecheck(
    bounds(
        __C: rkyv::validation::ArchiveContext,
    )
))]
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
    Tup(#[rkyv(omit_bounds)] ListVec<Self>),
    Enum(Identifier, ListVec<Identifier>),
    Struct(Identifier, #[rkyv(omit_bounds)] ListVec<(Identifier, Self)>),
    Variant(Identifier, #[rkyv(omit_bounds)] ListVec<(Identifier, Self)>),
    Fvector(Int, #[rkyv(omit_bounds)] Box<Self>),
    Vector(#[rkyv(omit_bounds)] Box<Self>),
    List(#[rkyv(omit_bounds)] Box<Self>),
    Ref(#[rkyv(omit_bounds)] Box<Self>),
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
    HaveException(Int),
    CurrentException(Int),
    ThrowLocation(Int),
    Channel(Channel, Int),
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
    Ite,
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

/// clexp

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
#[rkyv(deserialize_bounds(<__D as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source))]
#[rkyv(serialize_bounds(__S: Writer))]
#[rkyv(bytecheck(
    bounds(
        __C: rkyv::validation::ArchiveContext,
    )
))]
pub enum Expression {
    Id(Name, Type),
    Rmw(Name, Name, Type),
    Field(#[rkyv(omit_bounds)] Box<Self>, Identifier),
    Addr(#[rkyv(omit_bounds)] Box<Self>),
    Tuple(#[rkyv(omit_bounds)] Box<Self>, Int),
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
#[rkyv(deserialize_bounds(<__D as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source))]
#[rkyv(serialize_bounds(__S: Writer+ rkyv::ser::Allocator))]
#[rkyv(bytecheck(
    bounds(
        __C: rkyv::validation::ArchiveContext,
    )
))]
pub enum Value {
    Id(Name, Type),
    // enum member
    Member(Identifier, Type),
    Lit(Vl, Type),
    Tuple(#[rkyv(omit_bounds)] ListVec<Self>, Type),
    Struct(#[rkyv(omit_bounds)] ListVec<(Identifier, Self)>, Type),
    CtorKind(
        #[rkyv(omit_bounds)] Box<Self>,
        (Identifier, ListVec<Type>),
        Type,
    ),
    CtorUnwrap(
        #[rkyv(omit_bounds)] Box<Self>,
        (Identifier, ListVec<Type>),
        Type,
    ),
    TupleMember(#[rkyv(omit_bounds)] Box<Self>, Int, Int),
    Call(Op, #[rkyv(omit_bounds)] ListVec<Self>),
    Field(#[rkyv(omit_bounds)] Box<Self>, Identifier),
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
            Self::Member(_, _) => todo!(),
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
pub enum CReturn {
    One(Expression),
    Multi(ListVec<Expression>),
}

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
#[rkyv(deserialize_bounds(<__D as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source))]
#[rkyv(serialize_bounds(__S: Writer+ rkyv::ser::Allocator))]
#[rkyv(bytecheck(
    bounds(
        __C: rkyv::validation::ArchiveContext,
    )
))]
pub enum InstructionAux {
    Decl(Type, Name),
    Init(Type, Name, Value),
    Jump(Value, InternedString),
    Goto(InternedString),
    Label(InternedString),
    Funcall(CReturn, bool, (Identifier, ListVec<Type>), ListVec<Value>),
    Copy(Expression, Value),
    Clear(Type, Name),
    Undefined(Type),
    Exit(InternedString),
    End(Name),
    If(
        Value,
        #[rkyv(omit_bounds)] ListVec<Instruction>,
        #[rkyv(omit_bounds)] ListVec<Instruction>,
        Type,
    ),
    Block(#[rkyv(omit_bounds)] ListVec<Instruction>),
    TryBlock(#[rkyv(omit_bounds)] ListVec<Instruction>),
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
            InstructionAux::Funcall(cret, _, (_, parameter_types), parameters) => {
                match cret {
                    CReturn::One(expr) => {
                        visitor.visit_expression(expr);
                    }
                    CReturn::Multi(exprs) => {
                        exprs.iter().for_each(|expr| {
                            visitor.visit_expression(expr);
                        });
                    }
                }

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

/// Cdef_aux
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
pub enum DefinitionAux {
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

/// cdef
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
pub struct Definition {
    pub def: DefinitionAux,
    pub annot: DefinitionAnnotation,
}

impl Walkable for Definition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.def {
            DefinitionAux::Register(_, typ, instructions) => {
                visitor.visit_type(typ);
                instructions
                    .iter()
                    .for_each(|i| visitor.visit_instruction(i));
            }
            DefinitionAux::Type(type_definition) => visitor.visit_type_definition(type_definition),
            DefinitionAux::Let(_, types, instructions) => {
                types.iter().for_each(|(_, typ)| visitor.visit_type(typ));
                instructions
                    .iter()
                    .for_each(|i| visitor.visit_instruction(i));
            }
            DefinitionAux::Val(_, _, types, typ) => {
                types.iter().for_each(|typ| visitor.visit_type(typ));
                visitor.visit_type(typ)
            }
            DefinitionAux::Fundef(_, _, _, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            DefinitionAux::Startup(_, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            DefinitionAux::Finish(_, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            DefinitionAux::Pragma(_, _) => (),
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

type InstructionAnnotation = (Int, Location);

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

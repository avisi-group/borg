use {
    crate::dbt::x86::emitter::{
        BinaryOperationKind, CastOperationKind, ShiftOperationKind, TernaryOperationKind,
        UnaryOperationKind, X86Block,
    },
    alloc::vec::Vec,
    common::arena::Ref,
};

pub trait Emitter {
    type BlockRef;
    type NodeRef;
    type SymbolRef;

    fn constant(&mut self, val: u64, typ: Type) -> Self::NodeRef;
    fn create_bits(&mut self, value: Self::NodeRef, length: Self::NodeRef) -> Self::NodeRef;
    fn size_of(&mut self, value: Self::NodeRef) -> Self::NodeRef;
    fn create_tuple(&mut self, values: Vec<Self::NodeRef>) -> Self::NodeRef;
    fn access_tuple(&mut self, tuple: Self::NodeRef, index: usize) -> Self::NodeRef;

    fn unary_operation(&mut self, op: UnaryOperationKind) -> Self::NodeRef;
    fn binary_operation(&mut self, op: BinaryOperationKind) -> Self::NodeRef;
    fn ternary_operation(&mut self, op: TernaryOperationKind) -> Self::NodeRef;
    fn cast(&mut self, value: Self::NodeRef, typ: Type, kind: CastOperationKind) -> Self::NodeRef;
    fn bits_cast(
        &mut self,
        value: Self::NodeRef,
        length: Self::NodeRef,
        typ: Type,
        kind: CastOperationKind,
    ) -> Self::NodeRef;
    fn shift(
        &mut self,
        value: Self::NodeRef,
        amount: Self::NodeRef,
        kind: ShiftOperationKind,
    ) -> Self::NodeRef;

    fn bit_extract(
        &mut self,
        value: Self::NodeRef,
        start: Self::NodeRef,
        length: Self::NodeRef,
    ) -> Self::NodeRef;

    fn bit_insert(
        &mut self,
        target: Self::NodeRef,
        source: Self::NodeRef,
        start: Self::NodeRef,
        length: Self::NodeRef,
    ) -> Self::NodeRef;

    fn select(
        &mut self,
        condition: Self::NodeRef,
        true_value: Self::NodeRef,
        false_value: Self::NodeRef,
    ) -> Self::NodeRef;

    fn assert(&mut self, condition: Self::NodeRef, metadata: u64);

    fn get_flags(&mut self, operation: Self::NodeRef) -> Self::NodeRef;

    fn read_register(&mut self, offset: u64, typ: Type) -> Self::NodeRef;
    fn write_register(&mut self, offset: u64, value: Self::NodeRef);

    fn read_memory(&mut self, address: Self::NodeRef, typ: Type) -> Self::NodeRef;
    fn write_memory(&mut self, address: Self::NodeRef, value: Self::NodeRef);

    fn read_virt_variable(&mut self, symbol: Self::SymbolRef) -> Self::NodeRef;
    fn write_virt_variable(&mut self, symbol: Self::SymbolRef, value: Self::NodeRef);
    fn read_stack_variable(&mut self, offset: usize, typ: Type) -> Self::NodeRef;
    fn write_stack_variable(&mut self, offset: usize, value: Self::NodeRef);

    // returns the vector with the new element
    fn mutate_element(
        &mut self,
        vector: Self::NodeRef,
        index: Self::NodeRef,
        value: Self::NodeRef,
    ) -> Self::NodeRef;

    fn panic(&mut self, msg: &str);

    fn branch(
        &mut self,
        condition: Self::NodeRef,
        true_target: Self::BlockRef,
        false_target: Self::BlockRef,
    ) -> BlockOutcome;

    fn jump(&mut self, target: Self::BlockRef) -> BlockOutcome;

    // cleanup and return
    fn leave(&mut self);

    fn set_current_block(&mut self, block: Self::BlockRef);
    fn get_current_block(&self) -> Self::BlockRef;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Unsigned(u16),
    Signed(u16),
    Floating(u16),
    Bits,
    Tuple,
}

impl Type {
    pub fn width(&self) -> u16 {
        match self {
            Type::Unsigned(w) | Type::Signed(w) | Type::Floating(w) => *w,
            Type::Bits => 64, // todo: should this be the runtime length?
            Type::Tuple => todo!(),
        }
    }
}

/// Block translation outcome
#[derive(Debug, Clone)]
pub enum BlockOutcome {
    /// Static jump to the next block
    Static(Ref<X86Block>),
    /// Dynamic conditional branch to two blocks
    Dynamic(Ref<X86Block>, Ref<X86Block>),
    /// Function return
    Return,
    /// Panic
    Panic,
}

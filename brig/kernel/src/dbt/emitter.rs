pub trait Emitter {
    type BlockRef;
    type NodeRef;
    type SymbolRef;

    fn constant(&mut self, val: u64, typ: Type) -> Self::NodeRef;
    fn add(&mut self, lhs: Self::NodeRef, rhs: Self::NodeRef) -> Self::NodeRef;
    fn read_register(&mut self, offset: Self::NodeRef, typ: Type) -> Self::NodeRef;
    fn write_register(&mut self, offset: Self::NodeRef, value: Self::NodeRef);

    fn read_variable(&mut self, symbol: Self::SymbolRef) -> Self::NodeRef;
    fn write_variable(&mut self, symbol: Self::SymbolRef, value: Self::NodeRef);

    fn branch(
        &mut self,
        condition: Self::NodeRef,
        true_target: Self::BlockRef,
        false_target: Self::BlockRef,
    );
    fn jump(&mut self, target: Self::BlockRef);
    // cleanup and return
    fn leave(&mut self);

    fn set_current_block(&mut self, block: Self::BlockRef);
}

#[derive(Debug, Clone, Copy)]
pub struct Type {
    pub kind: TypeKind,
    pub width: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum TypeKind {
    Unsigned,
    Signed,
    Floating,
}

pub struct WrappedEmitter<E: Emitter> {
    subemitter: E,
}

impl<E: Emitter> Emitter for WrappedEmitter<E> {
    type BlockRef = E::BlockRef;
    type NodeRef = E::NodeRef;
    type SymbolRef = E::SymbolRef;

    fn constant(&mut self, val: u64, typ: Type) -> Self::NodeRef {
        self.subemitter.constant(val, typ)
    }

    fn add(&mut self, lhs: Self::NodeRef, rhs: Self::NodeRef) -> Self::NodeRef {
        self.subemitter.add(lhs, rhs)
    }

    fn read_register(&mut self, offset: Self::NodeRef, typ: Type) -> Self::NodeRef {
        self.subemitter.read_register(offset, typ)
    }

    fn write_register(&mut self, offset: Self::NodeRef, value: Self::NodeRef) {
        self.subemitter.write_register(offset, value);
    }

    fn read_variable(&mut self, symbol: Self::SymbolRef) -> Self::NodeRef {
        self.subemitter.read_variable(symbol)
    }

    fn write_variable(&mut self, symbol: Self::SymbolRef, value: Self::NodeRef) {
        self.subemitter.write_variable(symbol, value);
    }

    fn branch(
        &mut self,
        condition: Self::NodeRef,
        true_target: Self::BlockRef,
        false_target: Self::BlockRef,
    ) {
        self.subemitter.branch(condition, true_target, false_target);
    }

    fn jump(&mut self, target: Self::BlockRef) {
        self.subemitter.jump(target);
    }

    fn leave(&mut self) {
        self.subemitter.leave();
    }

    fn set_current_block(&mut self, block: Self::BlockRef) {
        self.subemitter.set_current_block(block);
    }
}

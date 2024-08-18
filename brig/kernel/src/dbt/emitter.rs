pub trait Emitter {
    type BlockRef;
    type NodeRef;

    fn constant(&mut self, val: u64, typ: Type) -> Self::NodeRef;
    fn read_register(&mut self, offset: Self::NodeRef, typ: Type) -> Self::NodeRef;
    fn add(&mut self, lhs: Self::NodeRef, rhs: Self::NodeRef) -> Self::NodeRef;
    fn write_register(&mut self, offset: Self::NodeRef, value: Self::NodeRef);

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

pub mod dbt {
    // nested module required to avoid issue with multiple `extern crate alloc`
    // statements and file used in both borealis and brig
    extern crate alloc;

    use {alloc::collections::BTreeMap, alloc::rc::Rc, block::Block, block_id::BlockId};

    mod block_id {
        use core::{
            fmt::{self, LowerHex},
            sync::atomic::{AtomicU64, Ordering},
        };

        #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
        pub struct BlockId(u64);

        impl Default for BlockId {
            fn default() -> Self {
                Self::new()
            }
        }

        impl BlockId {
            /// Creates a new, unique ID
            pub fn new() -> Self {
                static COUNTER: AtomicU64 = AtomicU64::new(0);

                let num = COUNTER.fetch_add(1, Ordering::Relaxed);

                if num == u64::MAX {
                    panic!("COUNTER overflowed");
                }

                Self(num)
            }
        }

        impl LowerHex for BlockId {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:x}", self.0)
            }
        }
    }

    mod block {
        extern crate alloc;

        use {super::BlockId, alloc::vec::Vec};

        pub struct Block {
            id: BlockId,
            // placeholder
            generated_code: Vec<u8>,
        }

        impl Block {
            pub fn new() -> Self {
                Self {
                    id: BlockId::new(),
                    generated_code: Vec::new(),
                }
            }

            pub fn id(&self) -> BlockId {
                self.id
            }
        }
    }

    pub mod emitter {
        use super::Type;

        pub trait Value {
            fn typ(&self) -> Type;
        }

        pub trait Emitter {
            type Node: Value;

            fn constant(&self, val: u64, typ: Type) -> Self::Node;
            fn read_register(&self, offset: Self::Node, typ: Type) -> Self::Node;
            fn add(&self, lhs: Self::Node, rhs: Self::Node) -> Self::Node;
            fn write_register(&self, offset: Self::Node, value: Self::Node);
        }
    }

    pub mod x86 {
        extern crate alloc;

        use super::emitter::Emitter;
        use super::emitter::Value;
        use super::Type;
        use alloc::rc::Rc;

        pub struct X86Emitter;

        #[derive(Clone)]
        pub struct X86NodeRef(Rc<X86Node>);

        impl X86NodeRef {
            fn kind(&self) -> &X86NodeKind {
                &self.0.kind
            }
        }

        struct X86Node {
            typ: Type,
            kind: X86NodeKind,
        }

        impl X86Node {
            fn as_operand(&self) -> X86Operand {
                todo!();
            }
        }

        struct X86Operand;

        enum X86NodeKind {
            Constant { value: u64, width: u16 },
            GuestRegister { offset: u64 },
            BinaryOperation { kind: X86BinaryOperationKind },
        }

        pub enum X86BinaryOperationKind {
            Add(X86NodeRef, X86NodeRef),
            Sub(X86NodeRef, X86NodeRef),
            Multiply(X86NodeRef, X86NodeRef),
            Divide(X86NodeRef, X86NodeRef),
            Modulo(X86NodeRef, X86NodeRef),
            And(X86NodeRef, X86NodeRef),
            Or(X86NodeRef, X86NodeRef),
            Xor(X86NodeRef, X86NodeRef),
            PowI(X86NodeRef, X86NodeRef),
            CompareEqual(X86NodeRef, X86NodeRef),
            CompareNotEqual(X86NodeRef, X86NodeRef),
            CompareLessThan(X86NodeRef, X86NodeRef),
            CompareLessThanOrEqual(X86NodeRef, X86NodeRef),
            CompareGreaterThan(X86NodeRef, X86NodeRef),
            CompareGreaterThanOrEqual(X86NodeRef, X86NodeRef),
        }

        impl Value for X86NodeRef {
            fn typ(&self) -> Type {
                self.0.typ
            }
        }

        impl Emitter for X86Emitter {
            type Node = X86NodeRef;

            fn constant(&self, value: u64, typ: Type) -> Self::Node {
                X86NodeRef(Rc::new(X86Node {
                    typ,
                    kind: X86NodeKind::Constant {
                        value,
                        width: typ.width,
                    },
                }))
            }

            fn read_register(&self, offset: Self::Node, typ: super::Type) -> Self::Node {
                match offset.kind() {
                    X86NodeKind::Constant { value, .. } => X86NodeRef(Rc::new(X86Node {
                        typ,
                        kind: X86NodeKind::GuestRegister { offset: *value },
                    })),
                    _ => panic!("help"),
                }
            }

            fn add(&self, lhs: Self::Node, rhs: Self::Node) -> Self::Node {
                X86NodeRef(Rc::new(X86Node {
                    typ: lhs.typ(),
                    kind: X86NodeKind::BinaryOperation {
                        kind: X86BinaryOperationKind::Add(lhs, rhs),
                    },
                }))
            }

            fn write_register(&self, offset: Self::Node, value: Self::Node) {
                todo!()
            }
        }

        mod x86_machine_code {
            enum X86Instruction {
                Mov {
                    source: X86Operand,
                    destination: X86Operand,
                },
            }

            enum X86Operand {}
        }
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

    pub struct Context<E> {
        blocks: BTreeMap<BlockId, Block>,
        current_block_id: BlockId,
        emitter: E,
    }

    impl<E: emitter::Emitter> Context<E> {
        pub fn new(emitter: E) -> Self {
            let initial_block = Block::new();
            let current_block_id = initial_block.id();

            let mut blocks = BTreeMap::new();
            blocks.insert(current_block_id, initial_block);

            Self {
                blocks,
                current_block_id,
                emitter,
            }
        }

        pub fn emitter(&self) -> &E {
            &self.emitter
        }
    }
}

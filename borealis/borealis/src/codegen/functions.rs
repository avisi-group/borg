//! GenC function generation from BOOM

use {
    crate::{
        boom::{
            control_flow::{ControlFlowBlock, Terminator},
            visitor::{Visitor, Walkable},
            Ast, Definition, Statement,
        },
        codegen::emit::Emit,
        genc_model::{self, HelperFunction},
    },
    common::{intern::InternedString, HashMap, HashSet},
    itertools::Itertools,
    once_cell::sync::Lazy,
    std::{cell::RefCell, fmt::Write, rc::Rc},
};

/// GenC builtin functions that do not need to be generated
static BUILTIN_FNS: Lazy<HashSet<InternedString>> = Lazy::new(|| {
    let names = ["trap"];
    HashSet::from_iter(names.into_iter().map(InternedString::from_static))
});

/// Pre-generated GenC functions in text form to be inserted
static PREGENERATED_FNS: Lazy<HashMap<InternedString, HelperFunction>> = Lazy::new(|| {
    let fns = [
        HelperFunction {
            name: "aset_X".into(),
            parameters: "uint8 n, uint64 value".into(),
            return_type: "void".into(),
            body: r#"
                if (n != 31) {
                    write_register_bank(reg_RB, n, value);
                }
                return;
            "#
            .into(),
        },
        HelperFunction {
            name: "aget_X".into(),
            parameters: "uint8 width, uint8 n".into(),
            return_type: "uint64".into(),
            body: r#"
                if (n == 31) {
                    return 0;
                }

                uint64 value = read_register_bank(reg_RB, n);

                if (width == 32) {
                    return (uint32)value;
                } else {
                    return value;
                }
            "#
            .into(),
        },
        HelperFunction {
            name: "vector_subrange_A".into(),
            parameters: "uint64 value, uint8 start, uint8 end".into(),
            return_type: "uint64".into(),
            body: r#"
                return (value >> start) & ((1 << (end - start + 1)) - 1);
            "#
            .into(),
        },
        HelperFunction {
            name: "update_fbits".into(),
            parameters: "uint64 op, uint64 n, uint64 bit".into(),
            return_type: "uint64".into(),
            body: r#"
                if ((bit & 1) == 1) {
                    return op | (bit << n);
                } else {
                    return op & ~(bit << n);
                }
            "#
            .into(),
        },
        HelperFunction {
            name: "raw_GetSlice_int".into(),
            parameters: "uint64 len, uint64 n, uint64 start".into(),
            return_type: "uint64".into(),
            body: r#"
                if (len == 64) {
                    return n;
                } else {
                    return (n >> start) & ((1 << len) - 1);
                }
            "#
            .into(),
        },
        HelperFunction {
            name: "slice".into(),
            parameters: "uint64 n, uint64 start, uint64 len".into(),
            return_type: "uint64".into(),
            body: r#"
                return raw_GetSlice_int(len, n, start);
            "#
            .into(),
        },
        HelperFunction {
            name: "sail_assert".into(),
            parameters: "uint64 value".into(),
            return_type: "void".into(),
            body: r#"
                if (!value) {
                    trap();
                }
                return;
            "#
            .into(),
        },
        HelperFunction {
            name: "UsingAArch32".into(),
            parameters: "".into(),
            return_type: "uint8".into(),
            body: r#"
                return 0;
            "#
            .into(),
        },
        HelperFunction {
            name: "AArch64_BranchAddr".into(),
            parameters: "uint64 vaddress".into(),
            return_type: "uint64".into(),
            body: r#"
                return vaddress;
            "#
            .into(),
        },
        HelperFunction {
            name: "Hint_Branch".into(),
            parameters: "uint32 typ".into(),
            return_type: "void".into(),
            body: r#"
                return;
            "#
            .into(),
        },
        HelperFunction {
            name: "PostDecode".into(),
            parameters: "".into(),
            return_type: "void".into(),
            body: r#"
                return;
            "#
            .into(),
        },
    ];

    HashMap::from_iter(
        fns.into_iter()
            .map(|f| (InternedString::from(f.name.clone()), f)),
    )
});

/// Generates GenC helper functions from all functions in a BOOM AST
pub fn generate_fns(
    ast: Rc<RefCell<Ast>>,
    initial_fns: Vec<InternedString>,
) -> Vec<HelperFunction> {
    let mut remaining_fns = initial_fns;
    let mut generated_fns = PREGENERATED_FNS.clone();

    while let Some(ident) = remaining_fns.pop() {
        // skip if already generated
        if generated_fns.contains_key(&ident) {
            continue;
        }

        let ast = ast.borrow();
        let Some(definition) = ast.functions.get(&ident) else {
            log::trace!("cannot generate GenC for unknown function {ident:?}");
            continue;
        };

        #[allow(unstable_name_collisions)]
        let generated = HelperFunction {
            name: ident.to_string(),

            parameters: definition
                .signature
                .parameters
                .borrow()
                .iter()
                .map(Emit::emit_string)
                .join(", "),

            return_type: definition.signature.return_type.emit_string(),

            body: generate_fn_body(definition.entry_block.clone()),
        };

        generated_fns.insert(ident, generated);

        remaining_fns.extend(
            definition
                .entry_block
                .get_functions()
                // ignore builtin functions
                .difference(&BUILTIN_FNS),
        );
    }

    generated_fns.into_values().collect()
}

/// Generates constants from all enum variants
pub fn generate_enums(ast: Rc<RefCell<Ast>>) -> HashMap<InternedString, (genc_model::Typ, u64)> {
    ast.borrow()
        .definitions
        .iter()
        .filter_map(|def| {
            if let Definition::Enum { name, variants } = def {
                Some((name, variants))
            } else {
                None
            }
        })
        .flat_map(|(name, variants)| variants.iter().map(|variant| enum_mangle(*name, *variant)))
        .zip(0..)
        .map(|(k, v)| (k, (genc_model::Typ::Uint32, v)))
        .collect()
}

/// Mangled enum variant constant name generator
pub fn enum_mangle(_name: InternedString, variant: InternedString) -> InternedString {
    format!("{variant}").into()
}

#[derive(Debug)]
struct Indent {
    buf: String,
    num: usize,
    whitespace: &'static str,
}

impl Indent {
    pub fn new(whitespace: &'static str) -> Self {
        Self {
            buf: whitespace.to_owned(),
            num: 1,
            whitespace,
        }
    }

    pub fn inc(&mut self) {
        self.num += 1;

        while self.buf.len() < self.num * self.whitespace.len() {
            self.buf += self.whitespace;
            assert!(self.buf.len() == self.num * self.whitespace.len());
        }
    }

    pub fn dec(&mut self) {
        self.num -= 1;
    }

    pub fn get(&self) -> &str {
        &self.buf[..self.num * self.whitespace.len()]
    }
}

fn generate_fn_body(entry_block: ControlFlowBlock) -> String {
    enum StackItem {
        Block {
            // the control flow block
            block: ControlFlowBlock,
            // whether the child blocks should be emitted or not
            recurse: bool,
        },
        Else,
        EndElse,
    }

    let mut buf = String::new();
    let mut stack = vec![StackItem::Block {
        block: entry_block,
        recurse: true,
    }];
    let mut indent = Indent::new("    ");

    // if a block is unconditional, emit the statements and go to the next block
    // if a block is conditional, emit an if, else branch, where the if and else
    // blocks are indented one more

    while let Some(item) = stack.pop() {
        let (block, recurse) = match item {
            StackItem::Block {
                block,
                recurse: _recurse,
            } => (block, _recurse),
            StackItem::Else => {
                indent.dec();
                buf += indent.get();
                buf += "} else {\n";
                indent.inc();
                continue;
            }
            StackItem::EndElse { .. } => {
                indent.dec();
                buf += indent.get();
                buf += "}\n";
                continue;
            }
        };

        // write current block statements to buf here
        block.statements().iter().for_each(|stmt| {
            if let Statement::TypeDeclaration { typ, .. } = &*stmt.borrow() {
                buf += indent.get();
                writeln!(buf, "// {typ:?}").unwrap();
            }

            if let Statement::Copy { expression, value } = &*stmt.borrow() {
                buf += indent.get();
                writeln!(buf, "// {expression:?} {value:?}").unwrap();
            }

            buf += indent.get();
            stmt.emit(&mut buf).unwrap();
            buf += "\n";
        });

        if recurse {
            match block.terminator() {
                Terminator::Return(value) => {
                    buf += indent.get();
                    buf += "return";

                    if let Some(value) = value {
                        buf += " ";
                        Rc::new(RefCell::new(value)).emit(&mut buf).unwrap();
                    }

                    buf += ";\n";
                }
                Terminator::Unconditional { target } => {
                    stack.push(StackItem::Block {
                        block: target,
                        recurse: true,
                    });
                }
                Terminator::Conditional {
                    condition,
                    target,
                    fallthrough,
                } => {
                    buf += indent.get();
                    buf += "if (";
                    Rc::new(RefCell::new(condition)).emit(&mut buf).unwrap();
                    buf += ") {\n";
                    indent.inc();

                    // set up stack for processing the rest of the if statement

                    if target.terminator().targets().len() == 1
                        && target.terminator().targets() == fallthrough.terminator().targets()
                    {
                        // both target and fallthrough have common child
                        stack.extend([
                            StackItem::Block {
                                block: target.terminator().targets()[0].clone(),
                                recurse: true,
                            },
                            StackItem::EndElse,
                            StackItem::Block {
                                block: fallthrough,
                                recurse: false,
                            },
                            StackItem::Else,
                            StackItem::Block {
                                block: target,
                                recurse: false,
                            },
                        ]);
                    } else {
                        stack.extend([
                            StackItem::EndElse,
                            StackItem::Block {
                                block: fallthrough,
                                recurse: true,
                            },
                            StackItem::Else,
                            StackItem::Block {
                                block: target,
                                recurse: true,
                            },
                        ]);
                    }
                }
            }
        }
    }

    buf
}

/// Determines whether the supplied function is a branch instruction
pub fn contains_write_pc(ast: Rc<RefCell<Ast>>, function_name: InternedString) -> bool {
    let borrow = ast.borrow();
    let Some(fn_def) = borrow.functions.get(&function_name) else {
        return false;
    };

    struct PcWritefinder {
        ast: Rc<RefCell<Ast>>,
        writes_pc: bool,
    }

    impl Visitor for PcWritefinder {
        fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
            {
                if let Statement::FunctionCall { name, .. } = &*node.borrow() {
                    if contains_write_pc(self.ast.clone(), *name) {
                        self.writes_pc = true;
                    }
                }
            }
            node.borrow().walk(self);
        }

        fn visit_value(&mut self, node: Rc<RefCell<crate::boom::Value>>) {
            if let crate::boom::Value::Identifier(ident) = &*node.borrow() {
                if ident.as_ref() == "reg_PC_target" {
                    self.writes_pc = true;
                }
            }
        }
    }

    let mut finder = PcWritefinder {
        ast: ast.clone(),
        writes_pc: false,
    };

    finder.visit_function_definition(fn_def);

    finder.writes_pc
}

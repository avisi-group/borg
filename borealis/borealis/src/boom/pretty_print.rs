//! BOOM AST pretty printing

use {
    crate::{
        boom::{
            control_flow::ControlFlowBlock, visitor::Visitor, Ast, Definition, FunctionDefinition,
            FunctionSignature, NamedType, Statement,
        },
        codegen::emit::Emit,
    },
    std::{
        cell::RefCell,
        collections::HashSet,
        fmt::Write,
        rc::Rc,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

const PADDING: &str = "  ";

/// Pretty-print BOOM AST
pub fn print_ast<W: Write>(w: &mut W, ast: Rc<RefCell<Ast>>) {
    let Ast {
        definitions,
        registers,
        functions,
    } = &*ast.borrow();

    let mut visitor = BoomPrettyPrinter::new(w);

    definitions
        .iter()
        .for_each(|def| visitor.visit_definition(def));

    registers.iter().for_each(|(name, typ)| {
        write!(visitor.writer, "register {name}: ").unwrap();
        typ.emit(visitor.writer).unwrap();
        writeln!(visitor.writer).unwrap();
    });

    functions
        .iter()
        .for_each(|(_, fundef)| visitor.visit_function_definition(fundef));
}

/// Pretty-print JIB statement
pub fn print_statement<W: Write>(w: &mut W, statement: Rc<RefCell<Statement>>) {
    let mut visitor = BoomPrettyPrinter::new(w);
    visitor.visit_statement(statement);
}

/// Pretty-print JIB AST
pub struct BoomPrettyPrinter<'writer, W> {
    indent: Rc<AtomicUsize>,
    writer: &'writer mut W,
    visited_blocks: HashSet<ControlFlowBlock>,
}

impl<'writer, W: Write> BoomPrettyPrinter<'writer, W> {
    /// Creates a new `BoomPrettyPrinter` with the supplied writer
    pub fn new(writer: &'writer mut W) -> Self {
        Self {
            indent: Rc::new(AtomicUsize::new(0)),
            writer,
            visited_blocks: HashSet::new(),
        }
    }
}

impl<'writer, W: Write> BoomPrettyPrinter<'writer, W> {
    fn prindent<T: AsRef<str>>(&mut self, s: T) {
        write!(
            self.writer,
            "{}{}",
            PADDING.repeat(self.indent.load(Ordering::SeqCst)),
            s.as_ref()
        )
        .unwrap();
    }

    fn prindentln<T: AsRef<str>>(&mut self, s: T) {
        self.prindent(s);
        writeln!(self.writer).unwrap();
    }

    fn indent(&self) -> IndentHandle {
        self.indent.fetch_add(1, Ordering::SeqCst);
        IndentHandle {
            indent: self.indent.clone(),
        }
    }
}

struct IndentHandle {
    indent: Rc<AtomicUsize>,
}

impl Drop for IndentHandle {
    fn drop(&mut self) {
        self.indent.fetch_sub(1, Ordering::SeqCst);
    }
}

impl<'writer, W: Write> Visitor for BoomPrettyPrinter<'writer, W> {
    fn visit_definition(&mut self, node: &Definition) {
        match node {
            Definition::Enum { name, variants } => {
                self.prindentln(format!("enum {name} {{"));

                {
                    let _h = self.indent();
                    variants
                        .iter()
                        .for_each(|id| self.prindentln(format!("{id},")));
                }

                self.prindentln("}");
            }
            Definition::Union { name, fields } => {
                self.prindentln(format!("union {name} {{"));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|NamedType { name, typ }| {
                        self.prindent(format!("{name}: "));
                        typ.emit(self.writer).unwrap();
                        writeln!(self.writer, ",").unwrap();
                    });
                }

                self.prindentln("}");
            }
            Definition::Struct { name, fields } => {
                self.prindentln(format!("struct {name} {{"));

                {
                    let _h = self.indent();
                    fields.iter().for_each(|NamedType { name, typ }| {
                        self.prindent(format!("{name}: "));
                        typ.emit(self.writer).unwrap();
                        writeln!(self.writer, ",").unwrap();
                    });
                }

                self.prindentln("}");
            }
            Definition::Pragma { key, value } => {
                self.prindent(format!("#{key} {value}"));
            }
            Definition::Let { bindings, body } => {
                self.prindent("let (");

                let mut bindings = bindings.iter();
                if let Some(NamedType { name, .. }) = bindings.next() {
                    write!(self.writer, "{name}").unwrap();
                }
                for NamedType { name, .. } in bindings {
                    write!(self.writer, ", ").unwrap();
                    write!(self.writer, "{name}").unwrap();
                }

                writeln!(self.writer, ") {{").unwrap();

                {
                    let _h = self.indent();
                    body.iter()
                        .for_each(|statement| self.visit_statement(statement.clone()));
                }

                writeln!(self.writer, "}}").unwrap();
            }
        }
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        self.visit_function_signature(&node.signature);

        {
            let _h = self.indent();
            self.visit_control_flow_block(&node.entry_block);
        }

        writeln!(self.writer, "}}").unwrap();
    }

    fn visit_function_signature(
        &mut self,
        FunctionSignature {
            name,
            parameters,
            return_type,
        }: &FunctionSignature,
    ) {
        self.prindent(format!("fn {}(", name));

        let mut parameters = parameters.iter();
        if let Some(param) = parameters.next() {
            param.emit(self.writer).unwrap();
        }
        for param in parameters {
            write!(self.writer, ", ").unwrap();
            param.emit(self.writer).unwrap();
        }

        write!(self.writer, ") -> ").unwrap();
        return_type.emit(self.writer).unwrap();
        writeln!(self.writer, " {{").unwrap();
    }

    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        self.prindent("");
        node.emit(self.writer).unwrap();
        writeln!(self.writer).unwrap();
    }

    fn is_block_visited(&mut self, block: &ControlFlowBlock) -> bool {
        self.visited_blocks.contains(block)
    }

    fn set_block_visited(&mut self, block: &ControlFlowBlock) {
        self.visited_blocks.insert(block.clone());
    }
}

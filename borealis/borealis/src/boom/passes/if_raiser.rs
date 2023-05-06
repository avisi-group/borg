use {
    crate::boom::{
        passes::Pass,
        visitor::{Visitor, Walkable},
        Ast, NodeKind, Statement,
    },
    common::intern::InternedString,
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug, Default)]
pub struct IfRaiser {
    stack: Vec<NodeKind>,
    gotos: Vec<(InternedString, usize)>,
}

impl IfRaiser {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for IfRaiser {
    fn run(&mut self, _ast: Rc<RefCell<Ast>>) {}
}

impl Visitor for IfRaiser {
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        self.stack.push(NodeKind::Statement(node.clone()));

        match *node.borrow() {
            Statement::Goto(label)
            | Statement::Jump { target: label, .. }
            | Statement::Label(label) => {
                self.gotos.push((label, self.stack.len()));
            }

            _ => (),
        }

        node.borrow().walk(self);

        self.stack.pop().unwrap();
    }
}

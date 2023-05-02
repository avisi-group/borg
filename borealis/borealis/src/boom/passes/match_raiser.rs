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
pub struct MatchRaiser {
    stack: Vec<NodeKind>,
    gotos: Vec<(InternedString, usize)>,
}

impl MatchRaiser {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for MatchRaiser {
    fn run(&mut self, ast: Rc<RefCell<Ast>>) {
        let ast = ast.borrow();
        let def = ast
            .functions
            .get(&("integer_arithmetic_addsub_immediate_decode".into()))
            .unwrap();

        // def.control_flow.as_dot(&mut std::io::stdout()).unwrap();

        self.visit_function_definition(def);

        for (label, depth) in &self.gotos {
            for _ in 0..*depth {
                print!(" ");
            }
            println!("{label}");
        }
    }
}

impl Visitor for MatchRaiser {
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

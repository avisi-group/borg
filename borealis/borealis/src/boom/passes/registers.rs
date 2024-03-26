//! Replace any access or assignment to PSTATE with a call to write/read
//! register

use {
    crate::boom::{
        control_flow::ControlFlowBlock, passes::Pass, visitor::Visitor, Ast, Expression, Statement,
        Value,
    },
    common::{identifiable::unique_id, intern::InternedString},
    std::{cell::RefCell, rc::Rc},
};

/// Replace any access or assignment to a register with a call to
/// `read_register` or `write_register`, respectively.
#[derive(Debug, Default)]
pub struct RegisterHandler {
    ast: Option<Rc<RefCell<Ast>>>,
}

impl RegisterHandler {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for RegisterHandler {
    fn name(&self) -> &'static str {
        "RegisterHandler"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        self.ast = Some(ast.clone());

        ast.borrow().functions.iter().for_each(|(_, def)| {
            PcHandler {
                ast: ast.clone(),
                entry_block: def.entry_block.clone(),
            }
            .visit_function_definition(def);

            self.visit_function_definition(def);
        });

        false
    }
}

impl Visitor for RegisterHandler {
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        let Statement::Copy { expression, value } = node.borrow().clone() else {
            return;
        };

        // if we are copying *from* a register
        if let Value::Identifier(ident) = *value.borrow() {
            if self
                .ast
                .as_ref()
                .unwrap()
                .borrow()
                .registers
                .contains_key(&ident)
            {
                handle_register_read(node.clone(), expression.clone(), ident);
            }
        }

        // if we are copying *into* a register
        if let Expression::Identifier(ident) = expression {
            if self
                .ast
                .as_ref()
                .unwrap()
                .borrow()
                .registers
                .contains_key(&ident)
            {
                handle_register_write(node.clone(), value.clone(), ident);
            }
        }
    }
}

fn handle_register_read(
    node: Rc<RefCell<Statement>>,
    expression: Expression,
    register_name: InternedString,
) {
    *node.borrow_mut() = Statement::FunctionCall {
        expression: Some(expression),
        name: "read_register".into(),
        arguments: vec![Rc::new(RefCell::new(Value::Identifier(mangle_register(
            register_name,
        ))))],
    }
}

fn handle_register_write(
    node: Rc<RefCell<Statement>>,
    value: Rc<RefCell<Value>>,
    register_name: InternedString,
) {
    *node.borrow_mut() = Statement::FunctionCall {
        expression: None,
        name: "write_register".into(),
        arguments: vec![
            Rc::new(RefCell::new(Value::Identifier(mangle_register(
                register_name,
            )))),
            value,
        ],
    }
}

struct PcHandler {
    ast: Rc<RefCell<Ast>>,
    entry_block: ControlFlowBlock,
}

impl Visitor for PcHandler {
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        let stmt = { node.borrow().clone() };
        match stmt {
            // if expression == "u_PC", replace copy with write_pc function call
            Statement::Copy {
                expression: Expression::Identifier(ident),
                value,
            } => {
                if ident.as_ref() == "u_PC" {
                    *node.borrow_mut() = Statement::FunctionCall {
                        expression: None,
                        name: "write_register".into(),
                        arguments: vec![
                            Rc::new(RefCell::new(Value::Identifier("reg_PC_target".into()))),
                            value.clone(),
                        ],
                    }
                } else if ident.as_ref() == "u__PC_changed" {
                    *node.borrow_mut() = Statement::Comment("PC_changed remove here".into());
                }
            }
            Statement::FunctionCall {
                expression: Some(Expression::Identifier(ident)),
                name,
                arguments,
            } => {
                if ident.as_ref() == "u_PC" {
                    // lookup return type
                    let typ = self
                        .ast
                        .borrow()
                        .functions
                        .get(&name)
                        .unwrap_or_else(|| panic!("could not get function {name}"))
                        .signature
                        .return_type
                        .clone();

                    // create local var with type
                    let (block, idx) = self.entry_block.find_statement(node.clone()).unwrap();
                    let mut statements = block.statements();
                    let var_name = format!("pc_temp{}", unique_id()).into();

                    statements.insert(
                        idx,
                        Statement::TypeDeclaration {
                            name: var_name,
                            typ,
                        }
                        .into(),
                    );

                    // assign function call to that local var
                    statements[idx + 1] = Statement::FunctionCall {
                        expression: Some(Expression::Identifier(var_name)),
                        name,
                        arguments,
                    }
                    .into();

                    // pass local var as param to write_pc
                    statements.insert(
                        idx + 2,
                        Statement::FunctionCall {
                            expression: None,
                            name: "write_register".into(),
                            arguments: vec![
                                Rc::new(RefCell::new(Value::Identifier("reg_PC_target".into()))),
                                Rc::new(RefCell::new(Value::Identifier(var_name))),
                            ],
                        }
                        .into(),
                    );

                    block.set_statements(statements);
                }
            }
            _ => (),
        }
    }
}

/// Mangle the register name
pub fn mangle_register(name: InternedString) -> InternedString {
    format!("reg_{name}").into()
}

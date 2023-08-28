//! Replace any access or assignment to PSTATE with a call to write/read
//! register

use {
    crate::{
        boom::{
            control_flow::ControlFlowBlock, visitor::Visitor, Ast, Expression, Statement, Value,
        },
        passes::Pass,
    },
    common::{identifiable::unique_id, intern::InternedString},
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug, Default)]
pub struct RegisterHandler;

impl RegisterHandler {
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
        ast.borrow().functions.iter().for_each(|(_, def)| {
            self.visit_function_definition(def);

            PcHandler {
                ast: ast.clone(),
                entry_block: def.entry_block.clone(),
            }
            .visit_function_definition(def);
        });

        false
    }
}

impl Visitor for RegisterHandler {
    fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
        let Statement::Copy { expression, value } = node.borrow().clone() else {
            return;
        };

        // if we are copying *from* PSTATE
        if let Value::Field { value, field_name } = value.borrow().clone() {
            if let Value::Identifier(ident) = value.borrow().clone() {
                if ident.as_ref() == "PSTATE" {
                    handle_register_read(node.clone(), expression.clone(), field_name);
                }
            }
        }

        // if we are copying *into* PSTATE
        if let Expression::Field { expression, field } = expression {
            if let Expression::Identifier(ident) = *expression {
                if ident.as_ref() == "PSTATE" {
                    handle_register_write(node.clone(), value.clone(), field);
                }
            };
        }
    }
}

fn handle_register_read(
    node: Rc<RefCell<Statement>>,
    expression: Expression,
    field_name: InternedString,
) {
    *node.borrow_mut() = Statement::FunctionCall {
        expression: Some(expression),
        name: "read_register".into(),
        arguments: vec![Rc::new(RefCell::new(Value::Identifier(
            format!("reg_{field_name}").into(),
        )))],
    }
}

fn handle_register_write(
    node: Rc<RefCell<Statement>>,
    value: Rc<RefCell<Value>>,
    field: InternedString,
) {
    *node.borrow_mut() = Statement::FunctionCall {
        expression: None,
        name: "write_register".into(),
        arguments: vec![
            Rc::new(RefCell::new(Value::Identifier(
                format!("reg_{field}").into(),
            ))),
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
            // if expression == "PC", replace copy with write_pc function call
            Statement::Copy {
                expression: Expression::Identifier(ident),
                value,
            } => {
                if ident.as_ref() == "PC" {
                    *node.borrow_mut() = Statement::FunctionCall {
                        expression: None,
                        name: "write_register".into(),
                        arguments: vec![
                            Rc::new(RefCell::new(Value::Identifier("reg_PC_target".into()))),
                            value.clone(),
                        ],
                    }
                } else if ident.as_ref() == "PC_changed" {
                    *node.borrow_mut() = Statement::Comment("PC_changed remove here".into());
                }
            }
            Statement::FunctionCall {
                expression: Some(Expression::Identifier(ident)),
                name,
                arguments,
            } => {
                if ident.as_ref() == "PC" {
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

        // if PC is being assigned to write_pc

        //
    }
}

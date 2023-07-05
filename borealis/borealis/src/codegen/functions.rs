//! GenC function generation from BOOM

use {
    crate::{
        boom::{
            control_flow::{ControlFlowBlock, Terminator},
            pretty_print::BoomPrettyPrinter,
            visitor::Visitor,
            Ast,
        },
        codegen::genc::Render,
        genc_model::HelperFunction,
    },
    common::intern::InternedString,
    itertools::Itertools,
    std::{cell::RefCell, collections::HashMap, rc::Rc},
};

/// Generates GenC helper functions from all functions in a BOOM AST
pub fn generate_fns(
    ast: Rc<RefCell<Ast>>,
    initial_fns: Vec<InternedString>,
) -> Vec<HelperFunction> {
    let mut remaining_fns = initial_fns;
    let mut generated_fns = HashMap::new();

    while let Some(ident) = remaining_fns.pop() {
        let ast = ast.borrow();
        let definition = ast
            .functions
            .get(&ident)
            .expect("cannot generate GenC for unknown function");

        #[allow(unstable_name_collisions)]
        let generated = HelperFunction {
            name: ident.to_string(),

            parameters: definition
                .signature
                .parameters
                .iter()
                .map(Render::render)
                .join(", "),

            return_type: definition.signature.return_type.render(),

            //TODO: make this work for all functions
            body: if ident.as_ref() == "integer_arithmetic_addsub_immediate_decode" {
                generate_fn_body(definition.entry_block.clone())
            } else {
                "return;".to_owned()
            },
        };

        generated_fns.insert(ident, generated);
    }

    generated_fns.into_values().collect()
}

fn generate_fn_body(entry_block: ControlFlowBlock) -> String {
    enum StackItem {
        Block(ControlFlowBlock),
        Else,
        EndElse,
    }

    let mut buf = String::new();
    let mut stack = vec![StackItem::Block(entry_block)];
    let mut indent = "    ".to_owned();

    // if a block is unconditional, emit the statements and go to the next block
    // if a block is conditional, emit an if, else branch, where the if and else
    // blocks are indented one more

    while let Some(item) = stack.pop() {
        let block = match item {
            StackItem::Block(block) => block,
            StackItem::Else => {
                indent.truncate(indent.len() - 4);
                buf += &indent;
                buf += "} else {\n";
                indent += "    ";
                continue;
            }
            StackItem::EndElse => {
                indent.truncate(indent.len() - 4);
                buf += &indent;
                buf += "}\n";
                continue;
            }
        };

        //TODO: write current block statements to buf here

        match block.terminator() {
            Terminator::Return | Terminator::Undefined => {
                buf += &indent;
                buf += "return;\n";
            }
            Terminator::Unconditional { target } => {
                stack.push(StackItem::Block(target));
            }
            Terminator::Conditional {
                condition,
                target,
                fallthrough,
            } => {
                let condition_str = {
                    let mut condition_buf = vec![];
                    BoomPrettyPrinter::new(&mut condition_buf).visit_value(&condition);
                    String::from_utf8(condition_buf).unwrap()
                };

                if condition_str == "exception" {
                    continue;
                }

                // if the condition is a variable, emit an assignment to it
                if let Some(ident) = condition.get_ident() {
                    buf += &indent;
                    buf += "uint8 ";
                    buf += ident.as_ref();
                    buf += " = 0;\n";
                }

                buf += &indent;
                buf += "if (";
                buf += &condition_str;
                buf += ") {\n";
                indent += "    ";

                // set up stack for processing the rest of the if statement

                stack.extend([
                    StackItem::EndElse,
                    StackItem::Block(fallthrough),
                    StackItem::Else,
                    StackItem::Block(target),
                ]);
            }
        }
    }

    buf += &indent;
    buf += "return;\n";

    buf
}

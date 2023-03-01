//! Instruction execute behaviour
//!
//! Compiles the JIB for a

use sail::jib::{CVal, ClExp, Ctyp, InstrAux, Name};

use {
    common::intern::InternedStringKey,
    sail::jib::{CDef, Instruction},
    std::collections::LinkedList,
};

/// Compiles the JIB AST of a function to an equivalent GenC function
pub fn jib_func_to_genc(function_identifier: InternedStringKey, jib: &LinkedList<CDef>) -> String {
    if function_identifier != "integer_arithmetic_addsub_immediate_decode".into() {
        return "".to_owned();
    }

    let Some(CDef::Fundef(ident, _, parameters, instructions)) = jib
        .iter()
        .find(|def| match def {
            CDef::Fundef(ident, _, _, _) => ident.get_string() == function_identifier,
            _ => false,
        }) else {
            panic!("failed to find function definition with identifier {function_identifier} in JIB AST");
        };

    print_instructions(instructions, 0);

    todo!()
}

fn print_instructions(instructions: &LinkedList<Instruction>, indent: usize) {
    for instr in instructions {
        match &instr.inner {
            InstrAux::Block(instructions) => {
                print_indent(indent);
                println!("block {{");

                print_instructions(instructions, indent + 1);

                print_indent(indent);
                println!("}}");
            }
            InstrAux::Decl(typ, name) => {
                print_indent(indent);
                print_name(name);
                print!(": ");
                print_ctyp(typ);
                println!();
            }
            InstrAux::Copy(exp, val) => {
                print_indent(indent);
                print_exp(exp);
                print!(" = ");
                print_val(val);
                println!();
            }
            InstrAux::Clear(_, name) => {
                print_indent(indent);
                print!("clear(");
                print_name(name);
                println!(")");
            }
            InstrAux::Funcall(exp, _, (name, _), args) => {
                print_indent(indent);
                print_exp(exp);
                print!(" = {}(", name.get_string());
                for arg in args {
                    print_val(arg);
                    print!(", ");
                }
                println!(")");
            }
            InstrAux::Goto(label) => {
                print_indent(indent);
                println!("goto {label}");
            }
            InstrAux::Label(label) => {
                print_indent(indent);
                println!("label {label}");
            }
            InstrAux::If(condition, zif, zelse, _) => {
                print_indent(indent);
                print!("if (");
                print_val(condition);
                println!(") {{");
                print_instructions(zif, indent + 1);
                print_indent(indent);
                println!("}} else {{");
                print_instructions(zelse, indent + 1);
                print_indent(indent);
                println!("}}");
            }
            _ => (),
        }
    }
}

fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("  ");
    }
}

fn print_name(name: &Name) {
    match name {
        Name::Global(ident, _) | Name::Name(ident, _) => print!("{}", ident.get_string()),
        _ => (),
    }
}

fn print_ctyp(ctyp: &Ctyp) {
    match ctyp {
        Ctyp::Variant(ident, _) => print!("{}", ident.get_string()),
        _ => print!("{:?}", ctyp),
    }
}

fn print_exp(exp: &ClExp) {
    match exp {
        ClExp::Id(name, _) => print_name(name),
        _ => (),
    }
}

fn print_val(val: &CVal) {
    match val {
        CVal::Id(name, _) => print_name(name),
        CVal::Lit(val, _) => print!("{val:?}"),
        CVal::Call(op, vals) => {
            print!("{op:?}(");
            for val in vals {
                print_val(val);
            }
            print!(")")
        }
        _ => (),
    }
}

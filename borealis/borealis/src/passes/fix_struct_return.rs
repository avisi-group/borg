//! Replace any access or assignment to PSTATE with a call to write/read
//! register

use {
    crate::{
        boom::{Ast, FunctionDefinition, NamedType, Type},
        passes::Pass,
    },
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug, Default)]
pub struct FixStructReturn;

impl FixStructReturn {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for FixStructReturn {
    fn name(&self) -> &'static str {
        "FixStructReturn"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .iter()
            .for_each(|(_, def)| fix_function(def));

        false
    }
}

fn fix_function(fn_def: &FunctionDefinition) {
    // find function with struct as the return type
    {
        let Type::Struct { .. } = &*fn_def.signature.return_type.borrow() else {
            return;
        };
    }

    // replace with void return type
    let struct_type = fn_def.signature.return_type.replace(Type::Unit);

    // insert ref paramater in pos 0 to struct
    fn_def.signature.parameters.borrow_mut().insert(
        0,
        NamedType {
            name: "return_value_struct_ref".into(),
            typ: Rc::new(RefCell::new(Type::Reference(Rc::new(RefCell::new(
                struct_type,
            ))))),
        },
    )

    //
}

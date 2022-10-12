//! Sail2 language parser

pub mod ast;
//pub mod error;
pub mod span;

ocaml::import! {
    fn hello_world() -> String;
}

pub fn entry_point() {
    let gc = ocaml::runtime::init();

    unsafe {
        println!("{}", hello_world(&gc).unwrap());
    }
}

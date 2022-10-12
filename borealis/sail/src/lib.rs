//! Sail2 language parser

use ocaml::List;

pub mod ast;
//pub mod error;
pub mod span;

ocaml::import! {
    fn hello_world() -> String;
    fn dedup(l: List<i32>) -> List<i32>;
}

pub fn entry_point() {
    let gc = ocaml::runtime::init();

    unsafe {
        let mut l = List::empty();
        l = l.add(&gc, &-413);
        l = l.add(&gc, &123);
        l = l.add(&gc, &8484747);
        l = l.add(&gc, &-413);

        println!("{}", hello_world(&gc).unwrap());
        println!("{:?}", dedup(&gc, l).unwrap().into_vec());
    }
}

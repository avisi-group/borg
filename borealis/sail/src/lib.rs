//! Sail2 language parser

use ocaml::List;

pub mod ast;
//pub mod error;
pub mod span;

ocaml::import! {
    fn dedup(l: List<i32>) -> List<i32>;
}

#[cfg(test)]
mod tests {
    use {crate::dedup, ocaml::List};

    #[test]
    fn util_dedup() {
        let gc = ocaml::runtime::init();

        unsafe {
            let mut l = List::empty();

            l = l.add(&gc, &-413);
            l = l.add(&gc, &123);
            l = l.add(&gc, &8484747);
            l = l.add(&gc, &-413);

            assert_eq!(vec![-413, 123, 8484747], dedup(&gc, l).unwrap().into_vec());
        }
    }
}

use {
    crate::boom::{Ast, Statement},
    log::trace,
    phf::{phf_map, Map},
};

pub static HANDLERS: Map<&str, fn(&Ast, &Statement, &str)> = phf_map! {
    "internal_pick" => internal_pick,
    "eq_anything" => eq_anything,
};

fn internal_pick(_ast: &Ast, _node: &Statement, typ: &str) {
    trace!("internal_pick<{}>", typ);
}

fn eq_anything(_ast: &Ast, _node: &Statement, typ: &str) {
    trace!("eq_anything<{}>", typ);
}

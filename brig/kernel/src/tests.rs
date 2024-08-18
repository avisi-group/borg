use {linkme::distributed_slice, proc_macro_lib::ktest};

#[distributed_slice]
pub static TESTS: [fn()];

pub fn run_all() {
    log::info!("running {} tests", TESTS.len());
    for test in TESTS {
        test();
    }
}

#[ktest]
fn smoke() {
    assert!(1 + 1 == 2);
}

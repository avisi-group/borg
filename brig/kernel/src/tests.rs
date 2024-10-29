use {linkme::distributed_slice, proc_macro_lib::ktest};

#[distributed_slice]
pub static TESTS: [(&str, fn())];

pub fn run(individual: Option<&str>) {
    if let Some(test_name) = individual {
        TESTS
            .iter()
            .find(|(name, _)| *name == test_name)
            .unwrap_or_else(|| panic!("no test named {test_name:?} found"))
            .1();
    } else {
        log::info!("running {} tests", TESTS.len());
        for (name, test) in TESTS {
            log::trace!("running {name:?}");
            test();
        }
    }
}

#[ktest]
fn smoke() {
    assert!(1 + 1 == 2);
}

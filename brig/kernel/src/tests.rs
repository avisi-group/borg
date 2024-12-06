use {alloc::vec::Vec, common::TestConfig, linkme::distributed_slice, proc_macro_lib::ktest};

#[distributed_slice]
pub static TESTS: [(&str, fn())];

pub fn run(config: TestConfig) {
    let tests = TESTS
        .iter()
        .filter(|(name, _)| match &config {
            TestConfig::None => false,
            TestConfig::Include(include) => include.iter().any(|s| s == *name),
            TestConfig::Exclude(exclude) => !exclude.iter().any(|s| s == *name),
            TestConfig::All => true,
        })
        .collect::<Vec<_>>();

    log::info!("running {} tests", tests.len());
    for (name, test) in tests {
        log::trace!("running {name:?}");
        test();
    }
}

#[ktest]
fn smoke() {
    assert!(1 + 1 == 2);
}

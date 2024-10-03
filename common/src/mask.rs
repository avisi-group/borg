pub fn mask<I: Into<u32>>(width: I) -> u64 {
    1u64.checked_shl(width.into())
        .map(|n| n - 1)
        .unwrap_or(u64::MAX)
}

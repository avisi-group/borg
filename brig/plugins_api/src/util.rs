use core::num::ParseIntError;

/// Parses `0x`-prefixed, underscore separated hexadecimal values (like a memory
/// address)
///
/// Shouldn't really live here, ideally in some common utility crate, but
/// `plugins_api` is sorta serving that purpose
pub fn parse_hex_prefix<S: AsRef<str>>(s: S) -> Result<u64, ParseIntError> {
    // remove any underscores
    let s = s.as_ref().replace('_', "");
    // remove prefix
    let s = s.trim_start_matches("0x");

    u64::from_str_radix(s, 16)
}

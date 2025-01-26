extern crate std;

use {
    byte_unit::{AdjustedByte, Byte},
    color_eyre::{eyre::WrapErr, Result},
    errctx::PathCtx,
    std::{fs::File, io::BufWriter, path::Path},
};

/// Initialize the logger
pub fn init_logger(filters: &str) -> Result<()> {
    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder.parse_filters(filters);
    builder.try_init().wrap_err("Failed to initialise logger")?;
    Ok(())
}

/// Creates the file supplied in `path`.
///
/// If the file at the supplied path already exists it will
/// be overwritten.
pub fn create_file_buffered<P: AsRef<Path>>(path: P) -> Result<BufWriter<File>> {
    File::options()
        .write(true) // we want to write to the file...
        .create(true) // ...creating if it does not exist..
        .truncate(true) // ...and truncate before writing
        .open(path.as_ref())
        .map(BufWriter::new)
        .map_err(PathCtx::f(path))
        .wrap_err("Failed to write to file")
}

/// Number of bytes to human-readable `Display`able
pub fn bytes(num: usize) -> AdjustedByte {
    Byte::from(num).get_appropriate_unit(byte_unit::UnitType::Binary)
}

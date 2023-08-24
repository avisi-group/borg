use {
    crate::genc_model::files::{write_header, Function},
    std::fmt::{self, Display, Formatter},
};

/// Behaviours GenC file
#[derive(Debug, Clone)]
pub struct Behaviours(pub Vec<Function>);

impl Display for Behaviours {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write_header(f)?;

        for func in &self.0 {
            writeln!(f, "{func}")?;
        }

        Ok(())
    }
}

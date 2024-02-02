use {
    crate::genc::{files::write_header, Function},
    std::fmt::{self, Display, Formatter},
};

/// Execution GenC file containing instruction implementations
#[derive(Debug, Clone)]
pub struct Execute(pub Vec<Function>);

impl Display for Execute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write_header(f)?;

        for func in &self.0 {
            writeln!(f, "{func}")?;
        }

        Ok(())
    }
}

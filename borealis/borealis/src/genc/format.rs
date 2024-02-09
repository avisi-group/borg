//! GenC instruction formatting

use {common::intern::InternedString, itertools::Itertools, std::fmt::Display};

/// Format describing the binary coding of an instruction
#[derive(Debug, Clone)]
pub struct InstructionFormat(pub Vec<Segment>);

impl Display for InstructionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(ToString::to_string).join(" "))
    }
}

/// Segment in a format
#[derive(Debug, Clone)]
pub struct Segment {
    /// Content of the segment
    pub content: SegmentContent,
    /// Length of the segment in bits
    pub length: usize,
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.content, self.length)
    }
}

/// Contnet of a segment
#[derive(Debug, Clone)]
pub enum SegmentContent {
    /// Variable name
    Variable(InternedString),
    /// Constant value
    ///
    /// * Will be truncated to the length described in the Segment.
    /// * `u64` should really be a `Vec<u8>` or `BigInt`, there should be no
    ///   technical limitation on size of a constant.
    Constant(u64),
}

impl Display for SegmentContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SegmentContent::Variable(s) => write!(f, "%{s}"),
            SegmentContent::Constant(x) => write!(f, "{x:#x}"),
        }
    }
}

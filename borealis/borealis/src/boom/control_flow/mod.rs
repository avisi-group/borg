//! Generating control flow graph from BOOM
//!
//! Needs to be restructured due to two main areas of complexity, both caused by
//! block targets being unresolved during building (IE. visiting a jump before
//! the label it references is created):
//!
//! 1. Two sets of types, internal maybe-unresolved and public resolved to
//! enforce resolution at type level. 2. Recursive resolution to convert
//! maybe-unresolved to resolved blocks.

pub mod dot;
pub mod util;

//! Interfaces for emulated guests

use {
    crate::object::Object,
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::{any::Any, fmt::Debug},
};

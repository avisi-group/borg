//! String interning

use {
    deepsize::DeepSizeOf,
    fxhash::FxBuildHasher,
    lasso::{Spur, ThreadedRodeo},
    ocaml::{FromValue, ToValue, Value},
    once_cell::sync::Lazy,
};

/// String interner instance
pub static INTERNER: Lazy<ThreadedRodeo<Spur, FxBuildHasher>> =
    Lazy::new(|| ThreadedRodeo::with_hasher(FxBuildHasher::default()));

/// Key for an interned string
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct InternedStringKey(Spur);

impl InternedStringKey {
    /// Create a new interned string
    pub fn new<A: AsRef<str>>(str: A) -> Self {
        Self(INTERNER.get_or_intern(str))
    }

    /// Create a new interned string from a static str
    pub fn from_static(key: &'static str) -> Self {
        Self(INTERNER.get_or_intern_static(key))
    }
}

impl From<Spur> for InternedStringKey {
    fn from(spur: Spur) -> Self {
        Self(spur)
    }
}

impl From<String> for InternedStringKey {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl From<&'_ str> for InternedStringKey {
    fn from(string: &str) -> Self {
        Self::new(string)
    }
}

impl std::fmt::Debug for InternedStringKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        INTERNER.resolve(&self.0).fmt(f)
    }
}

impl std::fmt::Display for InternedStringKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        INTERNER.resolve(&self.0).fmt(f)
    }
}

impl<'de> serde::Deserialize<'de> for InternedStringKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::new)
    }
}

impl serde::Serialize for InternedStringKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

unsafe impl FromValue for InternedStringKey {
    fn from_value(v: Value) -> Self {
        Self::new(String::from_value(v))
    }
}

unsafe impl ToValue for InternedStringKey {
    fn to_value(&self, rt: &ocaml::Runtime) -> Value {
        self.to_string().to_value(rt)
    }
}

impl DeepSizeOf for InternedStringKey {
    fn deep_size_of_children(&self, _context: &mut deepsize::Context) -> usize {
        0
    }

    fn deep_size_of(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

use {
    crate::HashMap,
    alloc::{
        borrow::ToOwned,
        string::{String, ToString},
    },
    core::{cell::OnceCell, hash::BuildHasherDefault},
    deepsize::DeepSizeOf,
    lasso::{Key, Spur},
    rkyv::{Archive, Archived, Fallible},
    twox_hash::XxHash64,
};

#[cfg(feature = "no-std")]
type Interner = lasso::Rodeo<Spur, BuildHasherDefault<XxHash64>>;

#[cfg(feature = "std")]
type Interner = lasso::ThreadedRodeo<Spur, BuildHasherDefault<XxHash64>>;

static mut INTERNER: OnceCell<Interner> = OnceCell::new();

fn interner() -> &'static mut Interner {
    unsafe {
        #[allow(static_mut_refs)]
        INTERNER.get_mut()
    }
    .unwrap()
}

/// Initializes the interner with an initial state
pub fn init(state: HashMap<String, u32>) {
    let interner = postcard::from_bytes(&postcard::to_allocvec(&state).unwrap()).unwrap();

    assert!(unsafe {
        #[allow(static_mut_refs)]
        INTERNER.set(interner)
    }
    .is_ok())
}

/// Gets the strings and associated keys of the current interner
pub fn get_interner_state() -> HashMap<String, u32> {
    interner()
        .strings()
        .map(|s| (s.to_owned(), InternedString::new(s).key()))
        .collect()
}

/// Key for an interned string
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct InternedString(Spur);

impl InternedString {
    /// Create a new interned string
    pub fn new<A: AsRef<str>>(str: A) -> Self {
        Self(interner().get_or_intern(str.as_ref()))
    }

    /// Create a new interned string from a static str
    pub fn from_static(key: &'static str) -> Self {
        Self(interner().get_or_intern_static(key))
    }

    /// Gets the inner key of the interned string
    pub fn key(&self) -> u32 {
        self.0.into_inner().into()
    }
}

impl AsRef<str> for InternedString {
    fn as_ref(&self) -> &str {
        interner().resolve(&self.0)
    }
}

impl From<Spur> for InternedString {
    fn from(spur: Spur) -> Self {
        Self(spur)
    }
}

impl From<String> for InternedString {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl From<&'_ str> for InternedString {
    fn from(string: &str) -> Self {
        Self::new(string)
    }
}

impl core::fmt::Debug for InternedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(interner().resolve(&self.0), f)
    }
}

impl core::fmt::Display for InternedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(interner().resolve(&self.0), f)
    }
}

impl<'de> serde::Deserialize<'de> for InternedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::new)
    }
}

impl serde::Serialize for InternedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(feature = "std")]
unsafe impl ocaml::FromValue for InternedString {
    fn from_value(v: ocaml::Value) -> Self {
        Self::new(String::from_value(v))
    }
}

#[cfg(feature = "std")]
unsafe impl ocaml::ToValue for InternedString {
    fn to_value(&self, rt: &ocaml::Runtime) -> ocaml::Value {
        self.to_string().to_value(rt)
    }
}

impl DeepSizeOf for InternedString {
    fn deep_size_of_children(&self, _context: &mut deepsize::Context) -> usize {
        0
    }

    fn deep_size_of(&self) -> usize {
        core::mem::size_of_val(self)
    }
}

impl rkyv::Archive for InternedString {
    type Archived = u32;

    type Resolver = ();

    unsafe fn resolve(&self, _pos: usize, _resolver: Self::Resolver, out: *mut Self::Archived) {
        out.write(self.key());
    }
}

impl<S: Fallible + rkyv::ser::Serializer> rkyv::Serialize<S> for InternedString {
    fn serialize(
        &self,
        _serializer: &mut S,
    ) -> Result<Self::Resolver, <S as rkyv::Fallible>::Error> {
        Ok(())
    }
}

impl<D: Fallible> rkyv::Deserialize<InternedString, D>
    for Archived<<InternedString as Archive>::Archived>
{
    fn deserialize(&self, _: &mut D) -> Result<InternedString, <D as Fallible>::Error> {
        Ok(InternedString::from(
            // try from usize adds 1, -1 to bring it back down
            // todo: why???
            Spur::try_from_usize(usize::try_from(*self).unwrap() - 1).unwrap(),
        ))
    }
}

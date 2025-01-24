use {
    alloc::string::String,
    deepsize::DeepSizeOf,
    lasso::{Key, Spur},
    rkyv::{
        rancor::{Fallible, Source},
        string::{ArchivedString, StringResolver},
        Archive, DeserializeUnsized, Place, SerializeUnsized,
    },
};

#[cfg(feature = "no-std")]
mod interner {
    use {
        crate::Hasher,
        core::hash::BuildHasherDefault,
        lasso::Spur,
        spin::{lazy::Lazy, mutex::Mutex},
    };

    static INTERNER: Lazy<Mutex<lasso::Rodeo<Spur, BuildHasherDefault<Hasher>>>> =
        Lazy::new(|| Mutex::new(lasso::Rodeo::with_hasher(Default::default())));

    pub fn get_or_intern(str: &str) -> Spur {
        INTERNER.lock().get_or_intern(str)
    }
    pub fn get_or_intern_static(str: &'static str) -> Spur {
        INTERNER.lock().get_or_intern_static(str)
    }

    pub fn resolve(key: Spur) -> &'static str {
        let interner = INTERNER.lock();
        let str = interner.resolve(&key);

        // SAFETY: INTERNER is static, keys are never dropped so this `&'interner str`
        // is really a `&'static str`
        unsafe { core::mem::transmute(str) }
    }
}

#[cfg(feature = "std")]
mod interner {
    extern crate std;

    use {crate::Hasher, core::hash::BuildHasherDefault, lasso::Spur, std::sync::LazyLock};

    static INTERNER: LazyLock<lasso::ThreadedRodeo<Spur, BuildHasherDefault<Hasher>>> =
        LazyLock::new(|| lasso::ThreadedRodeo::with_hasher(Default::default()));

    pub fn get_or_intern(str: &str) -> Spur {
        INTERNER.get_or_intern(str)
    }
    pub fn get_or_intern_static(str: &'static str) -> Spur {
        INTERNER.get_or_intern_static(str)
    }

    pub fn resolve(key: Spur) -> &'static str {
        INTERNER.resolve(&key)
    }
}

/// Key for an interned string
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct InternedString(Spur);

impl InternedString {
    /// Create a new interned string
    pub fn new<A: AsRef<str>>(str: A) -> Self {
        Self(interner::get_or_intern(str.as_ref()))
    }

    /// Create a new interned string from a static str
    pub fn from_static(key: &'static str) -> Self {
        Self(interner::get_or_intern_static(key))
    }

    pub fn key(&self) -> u32 {
        self.0.into_inner().into()
    }

    pub fn from_raw(key: u32) -> Self {
        Self(Spur::try_from_usize(key as usize).unwrap())
    }
}

impl AsRef<str> for InternedString {
    fn as_ref(&self) -> &str {
        interner::resolve(self.0)
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
        core::fmt::Debug::fmt(interner::resolve(self.0), f)
    }
}

impl core::fmt::Display for InternedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(interner::resolve(self.0), f)
    }
}

impl<'de> serde::Deserialize<'de> for InternedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <&str as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}

impl serde::Serialize for InternedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

#[cfg(feature = "ocaml")]
unsafe impl ocaml::FromValue for InternedString {
    fn from_value(v: ocaml::Value) -> Self {
        Self::new(String::from_value(v))
    }
}

#[cfg(feature = "ocaml")]
unsafe impl ocaml::ToValue for InternedString {
    fn to_value(&self, rt: &ocaml::Runtime) -> ocaml::Value {
        use alloc::string::ToString;
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

impl Archive for InternedString {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedString::resolve_from_str(self.as_ref(), resolver, out);
    }
}

impl<S: Fallible> rkyv::Serialize<S> for InternedString
where
    S::Error: Source,
    str: SerializeUnsized<S>,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, <S as Fallible>::Error> {
        ArchivedString::serialize_from_str(self.as_ref(), serializer)
    }
}

impl<D: Fallible> rkyv::Deserialize<InternedString, D> for ArchivedString
where
    str: DeserializeUnsized<str, D>,
{
    fn deserialize(&self, _: &mut D) -> Result<InternedString, <D as Fallible>::Error> {
        Ok(InternedString::from(self.as_str()))
    }
}

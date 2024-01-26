use {
    alloc::string::String, core::num::TryFromIntError, thiserror_core as thiserror,
    x86_64::structures::paging::page::AddressNotAligned,
};

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum Error {
    /// Virtio error: {0:?}
    Virtio(virtio_drivers::Error),
    /// Error when converting between `usize` and `u64`: {0:?}
    AddressTryFrom(TryFromIntError),
    /// Address not aligned when converted to page or frame: {0:?}
    AddressNotAligned(AddressNotAligned),
    /// JSON error: {0:?}
    Json(serde_json::Error),
    /// File was not found in the supplied TAR archive: {0:?}
    FileNotFoundInTar(String),
}

impl From<virtio_drivers::Error> for Error {
    fn from(value: virtio_drivers::Error) -> Self {
        Self::Virtio(value)
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Self::AddressTryFrom(value)
    }
}

impl From<AddressNotAligned> for Error {
    fn from(value: AddressNotAligned) -> Self {
        Self::AddressNotAligned(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

use core::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    _8,
    _16,
    _32,
    _64,
}

impl Width {
    pub fn from_uncanonicalized(bits: u16) -> Result<Self, WidthError> {
        match bits {
            1..=8 => Ok(Self::_8),
            9..=16 => Ok(Self::_16),
            17..=32 => Ok(Self::_32),
            33..=64 => Ok(Self::_64),
            0 => Err(WidthError::Zero),
            _ => Ok(Self::_64), // todo: fix PhysicalCount and other oversized registers
            n => Err(WidthError::Oversize(n)),
        }
    }

    fn as_u16(&self) -> u16 {
        match self {
            Width::_8 => 8,
            Width::_16 => 16,
            Width::_32 => 32,
            Width::_64 => 64,
        }
    }
}

impl Display for Width {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_u16())
    }
}

impl PartialOrd for Width {
    fn partial_cmp(&self, other: &Width) -> Option<Ordering> {
        self.as_u16().partial_cmp(&other.as_u16())
    }
}

#[derive(Debug, displaydoc::Display)]
/// Width canonicalization error
pub enum WidthError {
    /// Cannot encode 0 sized width
    Zero,
    /// Width {0} too large
    Oversize(u16),
}

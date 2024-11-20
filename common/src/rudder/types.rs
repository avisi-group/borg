use {
    alloc::{
        borrow::ToOwned,
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    },
    core::fmt::{self, Display, Formatter},
};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PrimitiveType {
    UnsignedInteger(u16),
    SignedInteger(u16),
    FloatingPoint(u16),
}

impl PrimitiveType {
    pub fn width(&self) -> u16 {
        match self {
            Self::FloatingPoint(w) | Self::SignedInteger(w) | Self::UnsignedInteger(w) => *w,
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Type {
    Primitive(PrimitiveType),

    Vector {
        element_count: usize,
        element_type: Box<Self>,
    },

    Tuple(Vec<Self>),

    Bits,

    // Used for debugging
    String,
}

macro_rules! type_def_helper {
    ($name: ident, $cls: ident, $width: expr) => {
        pub fn $name() -> Self {
            Self::new_primitive(PrimitiveType::$cls($width))
        }
    };
}

impl Type {
    pub fn new_primitive(primitive: PrimitiveType) -> Self {
        Self::Primitive(primitive)
    }

    /// Gets the offset in bytes of a field of a composite or an element of a
    /// vector
    pub fn byte_offset(&self, element_field: usize) -> Option<usize> {
        match self {
            Type::Vector { element_type, .. } => {
                Some(element_field * usize::try_from(element_type.width_bytes()).unwrap())
            }
            _ => None,
        }
    }

    pub fn width_bits(&self) -> u16 {
        match self {
            Self::Primitive(p) => p.width(),
            Self::Vector {
                element_count,
                element_type,
            } => u16::try_from(element_type.width_bits() as usize * *element_count).unwrap_or_else(
                |_| {
                    // todo: filter out oversized numbers earlier
                    // log::trace!(
                    //     "vector [{element_type};{element_count}] ({}) too big",
                    //     element_type.width_bits() as usize * *element_count
                    // );

                    0
                },
            ),

            Self::Bits => 64,

            // width of internedstring
            Self::String => 32,

            Self::Tuple(ts) => ts.iter().map(|typ| typ.width_bits()).sum(),
        }
    }

    pub fn width_bytes(&self) -> u16 {
        self.width_bits().div_ceil(8)
    }

    type_def_helper!(u1, UnsignedInteger, 1);
    type_def_helper!(u8, UnsignedInteger, 8);
    type_def_helper!(u16, UnsignedInteger, 16);
    type_def_helper!(u32, UnsignedInteger, 32);
    type_def_helper!(u64, UnsignedInteger, 64);
    type_def_helper!(u128, UnsignedInteger, 128);
    type_def_helper!(s8, SignedInteger, 8);
    type_def_helper!(s16, SignedInteger, 16);
    type_def_helper!(s32, SignedInteger, 32);
    type_def_helper!(s64, SignedInteger, 64);
    type_def_helper!(s128, SignedInteger, 128);
    type_def_helper!(f32, FloatingPoint, 32);
    type_def_helper!(f64, FloatingPoint, 64);

    pub fn vectorize(self, element_count: usize) -> Self {
        Self::Vector {
            element_count,
            element_type: Box::new(self),
        }
    }

    pub fn is_bits(&self) -> bool {
        matches!(self, Self::Bits)
    }

    pub fn is_u1(&self) -> bool {
        matches!(self, Self::Primitive(PrimitiveType::UnsignedInteger(1)))
    }

    pub fn is_unknown_length_vector(&self) -> bool {
        matches!(
            self,
            Self::Vector {
                element_count: 0,
                ..
            }
        )
    }

    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self == other
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Type::Primitive(p) => match &p {
                PrimitiveType::UnsignedInteger(_) => write!(f, "u{}", self.width_bits()),
                PrimitiveType::SignedInteger(_) => write!(f, "i{}", self.width_bits()),
                PrimitiveType::FloatingPoint(_) => write!(f, "f{}", self.width_bits()),
            },

            Type::Vector {
                element_count,
                element_type,
            } => write!(f, "[{element_type}; {element_count:?}]"),
            Type::Bits => write!(f, "bv"),
            Type::String => write!(f, "str"),
            Type::Tuple(ts) => {
                write!(f, "(").unwrap();
                for t in ts {
                    write!(f, "{t}, ").unwrap();
                }
                write!(f, ")")
            }
        }
    }
}

pub fn maybe_type_to_string(o: Option<Type>) -> String {
    o.as_ref()
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or("void".to_owned())
}

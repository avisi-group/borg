use common::intern::InternedString;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum PrimitiveTypeClass {
    Void,
    Unit,
    UnsignedInteger,
    SignedInteger,
    FloatingPoint,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct PrimitiveType {
    pub tc: PrimitiveTypeClass,
    pub element_width_in_bits: usize,
}

impl PrimitiveType {
    pub fn type_class(&self) -> PrimitiveTypeClass {
        self.tc
    }

    pub fn width(&self) -> usize {
        self.element_width_in_bits
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Struct(Vec<(InternedString, Type)>),

    Vector {
        element_count: usize,
        element_type: Box<Type>,
    },

    Tuple(Vec<Type>),

    // anything can be cast to/from a union value?
    Union {
        width: usize,
    },

    // ehhhh
    String,

    Bits,
    ArbitraryLengthInteger,
    Rational,

    // Any type, used for undefineds
    Any,
}

macro_rules! type_def_helper {
    ($name: ident, $cls: ident, $width: expr) => {
        pub fn $name() -> Self {
            Self::new_primitive(PrimitiveTypeClass::$cls, $width)
        }
    };
}

impl Type {
    pub fn new_primitive(tc: PrimitiveTypeClass, element_width: usize) -> Self {
        Self::Primitive(PrimitiveType {
            tc,
            element_width_in_bits: element_width,
        })
    }

    pub fn new_product(fields: Vec<(InternedString, Type)>) -> Self {
        Self::Struct(fields)
    }

    pub fn void() -> Self {
        Self::Primitive(PrimitiveType {
            tc: PrimitiveTypeClass::Void,
            element_width_in_bits: 0,
        })
    }

    pub fn unit() -> Self {
        Self::Primitive(PrimitiveType {
            tc: PrimitiveTypeClass::Unit,
            element_width_in_bits: 0,
        })
    }

    /// Gets the offset in bytes of a field of a composite or an element of a
    /// vector
    pub fn byte_offset(&self, element_field: usize) -> Option<usize> {
        match self {
            Type::Struct(fields) => Some(
                fields
                    .iter()
                    .take(element_field)
                    .fold(0, |acc, (_, typ)| acc + typ.width_bytes()),
            ),
            Type::Vector { element_type, .. } => Some(element_field * element_type.width_bytes()),
            _ => None,
        }
    }

    pub fn width_bits(&self) -> usize {
        match self {
            Self::Struct(xs) => xs.iter().map(|(_, typ)| typ.width_bits()).sum(),
            Self::Union { width } => *width,
            Self::Primitive(p) => p.element_width_in_bits,
            Self::Vector {
                element_count,
                element_type,
            } => element_type.width_bits() * element_count,

            Self::Bits | Self::ArbitraryLengthInteger => usize::try_from(u128::BITS).unwrap(),
            // width of internedstring
            Self::String => 32,
            Self::Rational => todo!(),
            Self::Any => todo!(),

            Self::Tuple(ts) => ts.iter().map(|typ| typ.width_bits()).sum(),
        }
    }

    pub fn width_bytes(&self) -> usize {
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

    pub fn is_void(&self) -> bool {
        match self {
            Self::Primitive(PrimitiveType { tc, .. }) => {
                matches!(tc, PrimitiveTypeClass::Void)
            }
            _ => false,
        }
    }

    pub fn is_unit(&self) -> bool {
        match self {
            Self::Primitive(PrimitiveType { tc, .. }) => {
                matches!(tc, PrimitiveTypeClass::Unit)
            }
            _ => false,
        }
    }

    pub fn is_bits(&self) -> bool {
        matches!(self, Self::Bits)
    }

    pub fn is_apint(&self) -> bool {
        matches!(self, Self::ArbitraryLengthInteger)
    }

    pub fn is_u1(&self) -> bool {
        match self {
            Self::Primitive(PrimitiveType {
                tc: PrimitiveTypeClass::UnsignedInteger,
                element_width_in_bits,
            }) => *element_width_in_bits == 1,
            _ => false,
        }
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

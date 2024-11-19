use {
    crate::intern::InternedString,
    alloc::vec::Vec,
    core::{
        cmp::Ordering,
        fmt::{self, Display, Formatter},
        ops::{Add, Div, Mul, Not, Sub},
    },
};

// idk why this is necessary
#[allow(unused_imports)]
use num_traits::float::FloatCore as _;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ConstantValue {
    UnsignedInteger(u64),
    SignedInteger(i64),
    FloatingPoint(f64),
    String(InternedString),
    Tuple(Vec<ConstantValue>),
}

impl ConstantValue {
    pub fn zero(&self) -> bool {
        match self {
            ConstantValue::UnsignedInteger(v) => *v == 0,
            ConstantValue::SignedInteger(v) => *v == 0,
            ConstantValue::FloatingPoint(v) => *v == 0.,

            ConstantValue::String(_) => false,
            ConstantValue::Tuple(_) => panic!(),
        }
    }

    pub fn smallest_width(&self) -> usize {
        match self {
            ConstantValue::UnsignedInteger(v) => (usize::BITS - v.leading_zeros()) as usize,
            ConstantValue::SignedInteger(v) => (isize::BITS - v.leading_zeros()) as usize,
            _ => panic!("can't figure out smallest width for this constant"),
        }
    }

    // pub fn zero_or_unit(&self) -> bool {
    //     match self {
    //         ConstantValue::UnsignedInteger(v) => *v == 0,
    //         ConstantValue::SignedInteger(v) => *v == 0,
    //         ConstantValue::FloatingPoint(v) => *v == 0.,
    //         ConstantValue::Unit => true,
    //     }
    // }

    pub fn is_unsigned(&self) -> bool {
        matches!(self, ConstantValue::UnsignedInteger(_))
    }

    pub fn is_signed(&self) -> bool {
        matches!(self, ConstantValue::SignedInteger(_))
    }

    pub fn powi(&self, i: ConstantValue) -> ConstantValue {
        let ConstantValue::SignedInteger(i) = i else {
            panic!();
        };

        match self {
            ConstantValue::FloatingPoint(f) => {
                let result = f.powi(i32::try_from(i).unwrap());

                // some sail source does actually want infinite/NaNs
                // if !result.is_finite() {
                //     panic!("got non-finite result {result} from {f}.powi({i})");
                // }

                ConstantValue::FloatingPoint(result)
            }

            _ => todo!(),
        }
    }
}

impl Add for ConstantValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ConstantValue::UnsignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                ConstantValue::UnsignedInteger(l + r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                ConstantValue::SignedInteger(l + i64::try_from(r).unwrap())
            }
            (ConstantValue::UnsignedInteger(l), ConstantValue::SignedInteger(r)) => {
                ConstantValue::SignedInteger(i64::try_from(l).unwrap() + r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::SignedInteger(r)) => {
                ConstantValue::SignedInteger(l + r)
            }
            (ConstantValue::FloatingPoint(l), ConstantValue::FloatingPoint(r)) => {
                ConstantValue::FloatingPoint(l + r)
            }
            (l, r) => panic!("invalid types for add: {l:?} {r:?}"),
        }
    }
}

impl Sub for ConstantValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ConstantValue::UnsignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                ConstantValue::UnsignedInteger(l - r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::SignedInteger(r)) => {
                ConstantValue::SignedInteger(l - r)
            }
            (ConstantValue::FloatingPoint(l), ConstantValue::FloatingPoint(r)) => {
                ConstantValue::FloatingPoint(l - r)
            }
            (l, r) => panic!("invalid types for sub: {l:?} {r:?}"),
        }
    }
}

impl Mul for ConstantValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ConstantValue::UnsignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                ConstantValue::UnsignedInteger(l * r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::SignedInteger(r)) => {
                ConstantValue::SignedInteger(l * r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                ConstantValue::SignedInteger(l * i64::try_from(r).unwrap())
            }
            (ConstantValue::FloatingPoint(l), ConstantValue::FloatingPoint(r)) => {
                ConstantValue::FloatingPoint(l * r)
            }
            (l, r) => panic!("invalid types for mul: {l:?} {r:?}"),
        }
    }
}

impl Div for ConstantValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ConstantValue::UnsignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                ConstantValue::UnsignedInteger(l / r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::SignedInteger(r)) => {
                ConstantValue::SignedInteger(l / r)
            }
            (ConstantValue::FloatingPoint(l), ConstantValue::FloatingPoint(r)) => {
                ConstantValue::FloatingPoint(l / r)
            }
            (l, r) => panic!("invalid types for div: {l:?} {r:?}"),
        }
    }
}

impl Not for ConstantValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ConstantValue::UnsignedInteger(v) => ConstantValue::UnsignedInteger(!v),
            ConstantValue::SignedInteger(v) => ConstantValue::SignedInteger(!v),
            ConstantValue::FloatingPoint(_)
            | ConstantValue::String(_)
            | ConstantValue::Tuple(_) => panic!("not a thing"),
        }
    }
}

impl PartialOrd for ConstantValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ConstantValue::UnsignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                l.partial_cmp(r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::SignedInteger(r)) => l.partial_cmp(r),
            (ConstantValue::FloatingPoint(l), ConstantValue::FloatingPoint(r)) => l.partial_cmp(r),

            _ => None,
        }
    }
}

impl Display for ConstantValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConstantValue::UnsignedInteger(v) => write!(f, "{v}u"),
            ConstantValue::SignedInteger(v) => write!(f, "{v}s"),
            ConstantValue::FloatingPoint(v) => write!(f, "{v}f"),
            ConstantValue::String(str) => write!(f, "{str:?}"),
            ConstantValue::Tuple(vs) => {
                write!(f, "(").unwrap();
                vs.iter().for_each(|v| write!(f, "{v},  ").unwrap());
                write!(f, ")")
            }
        }
    }
}

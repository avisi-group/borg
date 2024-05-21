use {
    common::intern::InternedString,
    std::{
        cmp::Ordering,
        ops::{Add, Div, Mul, Sub},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    UnsignedInteger(usize),
    SignedInteger(isize),
    FloatingPoint(f32),
    String(InternedString),
    Unit,
}

impl ConstantValue {
    pub fn zero(&self) -> bool {
        match self {
            ConstantValue::UnsignedInteger(v) => *v == 0,
            ConstantValue::SignedInteger(v) => *v == 0,
            ConstantValue::FloatingPoint(v) => *v == 0.,
            ConstantValue::Unit | ConstantValue::String(_) => false,
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
        match self {
            ConstantValue::UnsignedInteger(_) => true,
            _ => false,
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            ConstantValue::SignedInteger(_) => true,
            _ => false,
        }
    }

    pub fn powi(&self, i: ConstantValue) -> ConstantValue {
        let ConstantValue::FloatingPoint(f) = self else {
            panic!();
        };

        let ConstantValue::SignedInteger(i) = i else {
            panic!();
        };

        let result = f.powi(i32::try_from(i).unwrap());

        // some sail source does actually want infinite/NaNs
        // if !result.is_finite() {
        //     panic!("got non-finite result {result} from {f}.powi({i})");
        // }

        ConstantValue::FloatingPoint(result)
    }
}

impl Add for ConstantValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ConstantValue::UnsignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                ConstantValue::UnsignedInteger(l + r)
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

impl PartialOrd for ConstantValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ConstantValue::UnsignedInteger(l), ConstantValue::UnsignedInteger(r)) => {
                l.partial_cmp(r)
            }
            (ConstantValue::SignedInteger(l), ConstantValue::SignedInteger(r)) => l.partial_cmp(r),
            (ConstantValue::FloatingPoint(l), ConstantValue::FloatingPoint(r)) => l.partial_cmp(r),
            (ConstantValue::Unit, ConstantValue::Unit) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

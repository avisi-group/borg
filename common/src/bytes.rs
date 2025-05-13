use byte_unit::{AdjustedByte, Byte, UnitType};

pub fn bytes(n: usize) -> AdjustedByte {
    Byte::from(n).get_appropriate_unit(UnitType::Binary)
}

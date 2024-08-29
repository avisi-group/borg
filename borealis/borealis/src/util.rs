pub fn smallest_width_of_value(value: u64) -> u16 {
    u16::try_from((value + 1).next_power_of_two().ilog2()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::smallest_width_of_value;

    #[test]
    fn _0() {
        assert_eq!(smallest_width_of_value(0), 0);
    }

    #[test]
    fn _5() {
        assert_eq!(smallest_width_of_value(5), 3);
    }

    #[test]
    fn _8() {
        assert_eq!(smallest_width_of_value(8), 4);
    }
    #[test]
    fn u32max() {
        assert_eq!(smallest_width_of_value(u32::MAX as u64), 32);
    }
}

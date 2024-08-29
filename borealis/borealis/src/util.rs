pub fn smallest_width_of_value(value: i64) -> u16 {
    u16::try_from(
        (u64::try_from(value.abs()).unwrap() + 1)
            .next_power_of_two()
            .ilog2(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::smallest_width_of_value;

    #[test]
    fn _0() {
        assert_eq!(smallest_width_of_value(0), 0);
    }

    #[test]
    fn _neg1() {
        assert_eq!(smallest_width_of_value(-1), 1);
    }

    #[test]
    fn _neg2() {
        assert_eq!(smallest_width_of_value(-2), 2);
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
        assert_eq!(smallest_width_of_value(u32::MAX as i64), 32);
    }

    #[test]
    fn i32max() {
        assert_eq!(smallest_width_of_value(i32::MAX as i64), 31);
    }

    #[test]
    fn i32min() {
        assert_eq!(smallest_width_of_value(i32::MIN as i64), 32);
    }
}

#![allow(dead_code)]
mod bits;

// tests not in the module itself because we don't want them emitted
#[cfg(test)]
mod tests {
    use super::bits::Bits;

    #[test]
    fn sign_extend() {
        let bits = Bits::new(0xe57ba1c, 0x1c);
        let sign_extend = bits.sign_extend(64);
        assert_eq!(sign_extend.length(), 64);
        assert_eq!(sign_extend.value(), 0xfffffffffe57ba1c);
    }

    #[test]
    fn arithmetic_shift_right() {
        let bits = Bits::new(0xffff_ffd8 << 32, 0x40);
        let shift = bits.arithmetic_shift_right(32);
        assert_eq!(shift.length(), 64);
        assert_eq!(shift.value(), 0xffff_ffff_ffff_ffd8);
    }
}

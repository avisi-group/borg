use {proc_macro2::TokenStream, quote::quote};

pub type BitsValue = u128;
pub type BitsLength = u16;

pub fn codegen_bits() -> TokenStream {
    quote! {
            #[derive(Default, Clone, Copy, Debug)]
    pub struct Bits {
        value: u128,
        length: u16,
    }

    impl Bits {
        pub fn new(value: u128, length: u16) -> Self {
            Self { value, length }
        }

        pub fn value(&self) -> u128 {
            self.value
        }

        pub fn length(&self) -> u16 {
            self.length
        }
    }

    impl core::ops::Shl<i128> for Bits {
        type Output = Self;

        fn shl(self, rhs: i128) -> Self::Output {
            Self {
                value: self
                    .value()
                    .checked_shl(u32::try_from(rhs).unwrap())
                    .unwrap_or(0),
                length: self.length(),
            }
        }
    }

    impl core::ops::Shr<i128> for Bits {
        type Output = Self;

        fn shr(self, rhs: i128) -> Self::Output {
            Self {
                value: self
                    .value()
                    .checked_shr(u32::try_from(rhs).unwrap())
                    .unwrap_or(0),
                length: self.length(),
            }
        }
    }

    impl core::ops::Shl for Bits {
        type Output = Self;

        fn shl(self, rhs: Bits) -> Self::Output {
            Self {
                value: self
                    .value()
                    .checked_shl(u32::try_from(rhs.value()).unwrap())
                    .unwrap_or(0),
                length: self.length(),
            }
        }
    }

    impl core::ops::BitAnd for Bits {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value() & rhs.value(),
                length: self.length(),
            }
        }
    }

    impl core::ops::BitOr for Bits {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value() | rhs.value(),
                length: self.length(),
            }
        }
    }

    impl core::ops::BitXor for Bits {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value() ^ rhs.value(),
                length: self.length(),
            }
        }
    }

    impl core::ops::Add for Bits {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value().wrapping_add(rhs.value()),
                length: self.length(),
            }
        }
    }

    impl core::ops::Sub for Bits {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value().wrapping_sub(rhs.value()),
                length: self.length(),
            }
        }
    }

    impl core::ops::Not for Bits {
        type Output = Self;

        fn not(self) -> Self::Output {
            Self {
                value: !self.value(),
                length: self.length(),
            }
        }
    }

    impl core::cmp::PartialEq for Bits {
        fn eq(&self, other: &Self) -> bool {
            self.value() == other.value()
        }
    }

    impl core::cmp::Eq for Bits {}
        }
}

// pub fn codegen_int() -> TokenStream {
//     quote! {
//         #[derive(Default, Clone, Copy, Debug)]
//         pub struct Int {
//             value: i128,
//         }

//         impl Bits {
//             pub fn new(value: u128, length: u16) -> Self {
//                 Self {
//                     value,
//                     length,
//                 }
//             }

//             pub fn value(&self) -> u128 {
//                 self.value
//             }

//             pub fn length(&self) -> u16 {
//                 self.length
//             }

//             pub fn wrapping_add(self, rhs: Self) -> Self {
//                 let (value, overflow) = self.value().overflowing_add(rhs.value());

//                 Self {
//                     value,
//                     length: self.length(),
//                     overflow: self.overflow || rhs.overflow || overflow,
//                 }
//             }
//         }
//     }
// }

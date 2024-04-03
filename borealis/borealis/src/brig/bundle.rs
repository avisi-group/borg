use {proc_macro2::TokenStream, quote::quote};

pub fn codegen_bundle() -> TokenStream {
    quote! {
        #[derive(Default, Clone, Copy, Debug)]
        struct Bundle<V, L> {
            value: V,
            length: L,
        }

        impl<V: core::ops::BitAnd<Output = V>, L> core::ops::BitAnd for Bundle<V, L> {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value & rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::BitOr<Output = V>, L> core::ops::BitOr for Bundle<V, L> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value | rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::BitXor<Output = V>, L> core::ops::BitXor for Bundle<V, L> {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value ^ rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Shl<Output = V>, L> core::ops::Shl for Bundle<V, L> {
            type Output = Self;

            fn shl(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value << rhs.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Shr<Output = V>, L> core::ops::Shr for Bundle<V, L> {
            type Output = Self;

            fn shr(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value >> rhs.value,
                    length: self.length
                }
            }
        }

        impl<V, L> core::ops::Sub for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Sub<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value) - core::num::Wrapping(rhs.value)).0,
                    length: self.length
                }
            }
        }

        impl<V, L> core::ops::Add for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Add<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value) + core::num::Wrapping(rhs.value)).0,
                    length: self.length
                }
            }
        }

        impl<V, L> core::ops::Div for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Div<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value) / core::num::Wrapping(rhs.value)).0,
                    length: self.length
                }
            }
        }

        impl<V, L> core::ops::Mul for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Mul<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value) * core::num::Wrapping(rhs.value)).0,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Not<Output = V>, L> core::ops::Not for Bundle<V, L> {
            type Output = Self;
            fn not(self) -> Self {
                Self {
                    value: !self.value,
                    length: self.length
                }
            }
        }

        impl<V: core::ops::Neg<Output = V>, L> core::ops::Neg for Bundle<V, L> {
            type Output = Self;
            fn neg(self) -> Self {
                Self {
                    value: -self.value,
                    length: self.length
                }
            }
        }

        impl<V: core::cmp::PartialOrd, L> core::cmp::PartialOrd for Bundle<V, L> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }

        impl<V: core::cmp::PartialEq, L> core::cmp::PartialEq for Bundle<V, L> {
            fn eq(&self, other: &Self) -> bool {
                self.value.eq(&other.value)
            }
        }

        impl<V: core::cmp::PartialEq, L> core::cmp::Eq for Bundle<V, L> {}

        impl core::ops::Shr<Bundle<i64, u8>> for Bundle<u64, u8> {
            type Output = Self;

            fn shr(self, rhs: Bundle<i64, u8>) -> Self::Output {
                Self {
                    value:  self.value >> rhs.value,
                    length: self.length
                }
            }
        }

        impl core::ops::Shl<Bundle<i64, u8>> for Bundle<u64, u8> {
            type Output = Self;

            fn shl(self, rhs: Bundle<i64, u8>) -> Self::Output {
                Self {
                    value:  self.value << rhs.value,
                    length: self.length
                }
            }
        }

        impl core::ops::Add<Bundle<i64, u8>> for Bundle<u64, u8> {
            type Output = Self;

            fn add(self, rhs: Bundle<i64, u8>) -> Self::Output {
                let value = match rhs.value.is_positive() {
                    true => {
                        self.value + u64::try_from(rhs.value.abs()).unwrap()
                    },
                    false => {
                        self.value - u64::try_from(rhs.value.abs()).unwrap()
                    }
                };
                Self {
                    value,
                    length: self.length
                }
            }
        }
    }
}

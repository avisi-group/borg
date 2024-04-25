use {proc_macro2::TokenStream, quote::quote};

pub fn codegen_bundle() -> TokenStream {
    quote! {
        #[derive(Default, Clone, Copy, Debug)]
        pub struct Bundle<V, L> {
            value: V,
            length: L,
            overflow: bool,
        }

        impl<V: Copy, L: Copy> Bundle<V, L> {
            pub fn new(value: V, length: L) -> Self {
                Self {
                    value,
                    length,
                    overflow: false,
                }
            }

            pub fn value(&self) -> V {
                self.value
            }

            pub fn length(&self) -> L {
                self.length
            }
        }

        impl Bundle<u64, u8> where {
            pub fn wrapping_add(self, rhs: Self) -> Self {
                let (value, overflow) = self.value().overflowing_add(rhs.value());

                Self {
                    value,
                    length: self.length(),
                    overflow: self.overflow || rhs.overflow || overflow,
                }
            }
        }

        impl Bundle<i64, u8> where {
            pub fn wrapping_add(self, rhs: Self) -> Self {
                let (value, overflow) = self.value().overflowing_add(rhs.value());

                Self {
                    value,
                    length: self.length(),
                   overflow: self.overflow || rhs.overflow || overflow,
                }
            }
        }


        impl Bundle<i64, u8>  {
            pub fn abs(self) -> Self {
               Self {
                    value: self.value().abs(),
                    length: self.length(),
                    overflow: false,
               }
            }

            pub fn pow2(self) -> Self {
                Self {
                    value: self.value().pow(2),
                    length: self.length(),
                    overflow: false,
               }
            }
        }

        impl core::ops::Add<Bundle<i64, u8>> for Bundle<u64, u8> {
            type Output = Self;

            fn add(self, rhs: Bundle<i64, u8>) -> Self::Output {
                let value = match rhs.value().is_positive() {
                    true => {
                        self.value() + u64::try_from(rhs.value().abs()).unwrap()
                    },
                    false => {
                        self.value() - u64::try_from(rhs.value().abs()).unwrap()
                    }
                };
                Self {
                    value,
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl core::ops::Shl for Bundle<u64, u8> {
            type Output = Self;

            fn shl(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value().checked_shl(u32::try_from(rhs.value()).unwrap()).unwrap_or(0),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl core::ops::Shl for Bundle<i64, u8> {
            type Output = Self;

            fn shl(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value().checked_shl(u32::try_from(rhs.value()).unwrap()).unwrap_or(0),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: core::ops::BitAnd<Output = V> + Copy, L: Copy> core::ops::BitAnd for Bundle<V, L> {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value() & rhs.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: core::ops::BitOr<Output = V> + Copy, L: Copy> core::ops::BitOr for Bundle<V, L> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value() | rhs.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: core::ops::BitXor<Output = V> + Copy, L: Copy> core::ops::BitXor for Bundle<V, L> {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value() ^ rhs.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }



        impl<V: core::ops::Shr<Output = V> + Copy, L: Copy> core::ops::Shr for Bundle<V, L> {
            type Output = Self;

            fn shr(self, rhs: Self) -> Self::Output {
                Self {
                    value:  self.value() >> rhs.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: Copy, L: Copy> core::ops::Sub for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Sub<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value()) - core::num::Wrapping(rhs.value())).0,
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: Copy, L: Copy> core::ops::Div for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Div<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value()) / core::num::Wrapping(rhs.value())).0,
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: Copy, L: Copy> core::ops::Rem for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Rem<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value()) % core::num::Wrapping(rhs.value())).0,
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: Copy, L: Copy> core::ops::Mul for Bundle<V, L> where core::num::Wrapping<V>: core::ops::Mul<Output = core::num::Wrapping<V>> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    value: (core::num::Wrapping(self.value()) * core::num::Wrapping(rhs.value())).0,
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: core::ops::Not<Output = V> + Copy, L: Copy> core::ops::Not for Bundle<V, L> {
            type Output = Self;
            fn not(self) -> Self {
                Self {
                    value: !self.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: core::ops::Neg<Output = V> + Copy, L: Copy> core::ops::Neg for Bundle<V, L> {
            type Output = Self;
            fn neg(self) -> Self {
                Self {
                    value: -self.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl<V: core::cmp::PartialOrd + Copy, L: Copy> core::cmp::PartialOrd for Bundle<V, L> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.value().partial_cmp(&other.value())
            }
        }

        impl<V: core::cmp::PartialEq + Copy, L: Copy> core::cmp::PartialEq for Bundle<V, L> {
            fn eq(&self, other: &Self) -> bool {
                self.value() == other.value() && self.overflow == other.overflow
            }
        }

        impl<V: core::cmp::PartialEq + Copy, L: Copy> core::cmp::Eq for Bundle<V, L> {}

        impl core::ops::Shr<Bundle<i64, u8>> for Bundle<u64, u8> {
            type Output = Self;

            fn shr(self, rhs: Bundle<i64, u8>) -> Self::Output {
                Self {
                    value:  self.value() >> rhs.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }

        impl core::ops::Shl<Bundle<i64, u8>> for Bundle<u64, u8> {
            type Output = Self;

            fn shl(self, rhs: Bundle<i64, u8>) -> Self::Output {
                Self {
                    value:  self.value() << rhs.value(),
                    length: self.length(),
                    overflow: false,
                }
            }
        }


    }
}

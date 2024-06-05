use {
    crate::{
        brig::codegen_ident,
        rudder::{Context, RegisterDescriptor},
    },
    proc_macro2::{Literal, TokenStream},
    quote::{format_ident, quote},
};

/// Code generation for ISA state
pub fn codegen_state(rudder: &Context) -> TokenStream {
    // generate constant for each register offset
    let register_offsets = rudder
        .get_registers()
        .into_iter()
        .map(|(name, RegisterDescriptor { offset, .. })| {
            let name = format_ident!(
                "REG_{}",
                codegen_ident(name.as_ref().into())
                    .to_string()
                    .to_ascii_uppercase()
            );
            quote!(pub const #name: usize = #offset;)
        })
        .collect::<TokenStream>();

    // generate mapping from offset to name
    let register_name_map_contents = {
        let mut offset_names = rudder
            .get_registers()
            .into_iter()
            .map(|(name, RegisterDescriptor { offset, .. })| (offset, name))
            .collect::<Vec<_>>();

        offset_names.sort_by_key(|(offset, _)| *offset);

        offset_names
            .into_iter()
            .map(|(offset, name)| {
                let name = name.as_ref();

                quote!((#offset, #name),)
            })
            .collect::<TokenStream>()
    };

    let registers_len = rudder
        .get_registers()
        .into_values()
        .map(|RegisterDescriptor { typ, offset, .. }| offset + typ.width_bytes())
        .max()
        .unwrap();

    // let register_inits = rudder.get_registers()

    quote! {
        // todo check this is necessary
        #[repr(align(8))]
        pub struct State {
            data: [u8; #registers_len],
            guest_memory_base: usize,
        }

        impl State {
            /// Returns the ISA state with initial values and configuration set
            pub fn new(guest_memory_base: usize) -> Self {
                Self {
                    data: [0; #registers_len],
                    guest_memory_base,
                }
            }

            pub fn write_register<T: ToBytes>(&mut self, offset: usize, value: T) {
                let start = offset;
                let end = start + core::mem::size_of::<T>();
                self.data[start..end].copy_from_slice(value.to_bytes().as_ref());
            }

            pub fn read_register<T: FromBytes>(&self, offset: usize) -> T {
                let start = offset;
                let end = start + core::mem::size_of::<T>();
                T::from_bytes(&self.data[start..end])
            }

            pub fn guest_memory_base(&self) -> usize {
                self.guest_memory_base
            }
        }

        pub trait ToBytes {
            fn to_bytes(&self) -> impl AsRef<[u8]>;
        }


        impl ToBytes for u8 {
            fn to_bytes(&self) -> impl AsRef<[u8]> {
                self.to_be_bytes()
            }
        }
        impl ToBytes for u16 {
            fn to_bytes(&self) -> impl AsRef<[u8]> {
                self.to_be_bytes()
            }
        }
        impl ToBytes for u32 {
            fn to_bytes(&self) -> impl AsRef<[u8]> {
                self.to_be_bytes()
            }
        }
        impl ToBytes for u64 {
            fn to_bytes(&self) -> impl AsRef<[u8]> {
                self.to_be_bytes()
            }
        }
        impl ToBytes for u128 {
            fn to_bytes(&self) -> impl AsRef<[u8]> {
                self.to_be_bytes()
            }
        }
        impl ToBytes for i128 {
            fn to_bytes(&self) -> impl AsRef<[u8]> {
                self.to_be_bytes()
            }
        }

        pub trait FromBytes {
            fn from_bytes(bytes: &[u8]) -> Self;
        }



        impl FromBytes for u8 {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize / 8];
                buf.copy_from_slice(bytes);
                Self::from_be_bytes(buf)
            }
        }
        impl FromBytes for u16 {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize / 8];
                buf.copy_from_slice(bytes);
                Self::from_be_bytes(buf)
            }
        }
        impl FromBytes for u32 {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize / 8];
                buf.copy_from_slice(bytes);
                Self::from_be_bytes(buf)
            }
        }
        impl FromBytes for u64 {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize / 8];
                buf.copy_from_slice(bytes);
                Self::from_be_bytes(buf)
            }
        }

        impl FromBytes for i64 {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize / 8];
                buf.copy_from_slice(bytes);
                Self::from_be_bytes(buf)
            }
        }

        impl FromBytes for [u8; 32] {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut buf = [0u8; 32];
                buf.copy_from_slice(bytes);
                buf
            }
        }

        #register_offsets

        pub const REGISTER_NAME_MAP: &[(usize, &str)] = &[#register_name_map_contents];
    }
}

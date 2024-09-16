use {
    crate::{
        codegen::codegen_ident,
        rudder::{Model, RegisterDescriptor},
    },
    proc_macro2::TokenStream,
    quote::{format_ident, quote},
};

/// Code generation for ISA state
pub fn codegen_state(rudder: &Model) -> TokenStream {
    // generate constant for each register offset
    let register_offsets = rudder
        .get_registers()
        .into_iter()
        .map(|(name, RegisterDescriptor { offset, .. })| {
            let name = format_ident!(
                "REG_{}",
                codegen_ident(name.as_ref().into()).to_string().to_ascii_uppercase()
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

    quote! {
        const REGISTER_NAME_MAP: &[(usize, &str)] = &[#register_name_map_contents];

        #[repr(align(8))]
        pub struct State {
            data: [u8; #registers_len],
            guest_environment: alloc::boxed::Box<dyn plugins_api::guest::Environment>,
        }

        impl State {
            /// Returns the ISA state with initial values and configuration set
            pub fn new(guest_environment: alloc::boxed::Box<dyn plugins_api::guest::Environment>) -> Self {
                Self {
                    data: [0; #registers_len],
                    guest_environment,
                }
            }

            pub fn write_register<T>(&mut self, offset: usize, value: T) {
                let start = offset;
                let end = start + core::mem::size_of::<T>();
                unsafe { core::ptr::write_unaligned(self.data[start..end].as_mut_ptr().cast(), value) };
            }

            pub fn read_register<T>(&self, offset: usize) -> T {
                let start = offset;
                let end = start + core::mem::size_of::<T>();
                unsafe { core::ptr::read_unaligned(self.data[start..end].as_ptr().cast()) }
            }

            pub fn write_memory(&self, address: u64, data: &[u8]) {
                self.guest_environment.write_memory(address, data);
            }

            pub fn read_memory(&self, address: u64, data: &mut [u8]) {
                self.guest_environment.read_memory(address, data);
            }
        }

        impl core::fmt::Debug for State {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                writeln!(f, "State {{")?;
                for window in REGISTER_NAME_MAP.windows(2) {

                    let (offset, name) = window[0];
                    let (next_offset, _) = window[1];

                    write!(f, "{name}: 0x")?;

                    for byte_idx in 0..(next_offset - offset) {
                        write!(f, "{:x}", self.read_register::<u8>(offset + byte_idx))?;
                    }
                    writeln!(f)?;
                }
                writeln!(f, "}}")
            }
        }

        #register_offsets

        pub struct RegisterOffset {
            /// Name of the register
            pub name: &'static str,
            /// Offset in bytes inside the register
            pub offset: usize
        }

        pub fn lookup_register_by_offset(offset: usize) -> Option<RegisterOffset> {
            if offset > core::mem::size_of::<State>() {
                return None;
            }

            Some(match REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
                // found start of register
                Ok(idx) => {
                    RegisterOffset {
                        name: REGISTER_NAME_MAP[idx].1,
                        offset: 0,
                    }
                }
                // we're accessing inside a register
                Err(idx) => {
                    // get the register and print the offset from the base
                    let (register_offset, name) = REGISTER_NAME_MAP[idx - 1];

                    RegisterOffset {
                        name,
                        offset: offset - register_offset,
                    }
                }
            })
        }
    }
}

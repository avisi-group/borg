use {
    crate::{
        codegen::codegen_ident,
        rudder::{Context, RegisterDescriptor},
    },
    proc_macro2::TokenStream,
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

    quote! {
        #[repr(align(8))]
        pub struct State {
            data: [u8; #registers_len],
            interpreter_host: alloc::boxed::Box<dyn plugins_api::InterpreterHost>,
        }

        impl State {
            /// Returns the ISA state with initial values and configuration set
            pub fn new(interpreter_host: alloc::boxed::Box<dyn plugins_api::InterpreterHost>) -> Self {
                Self {
                    data: [0; #registers_len],
                    interpreter_host,
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
                self.interpreter_host.write_memory(address, data);
            }

            pub fn read_memory(&self, address: u64, data: &mut [u8]) {
                self.interpreter_host.read_memory(address, data);
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
            const REGISTER_NAME_MAP: &[(usize, &str)] = &[#register_name_map_contents];

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

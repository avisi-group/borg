use {
    crate::{brig::codegen_ident, rudder::Context},
    proc_macro2::{Literal, TokenStream},
    quote::{format_ident, quote},
};

/// Code generation for ISA state
pub fn codegen_state(rudder: &Context) -> TokenStream {
    // generate constant for each register offset
    let register_offsets = rudder
        .get_registers()
        .into_iter()
        .map(|(name, (_, offset))| {
            let name = format_ident!(
                "REG_{}",
                codegen_ident(name.as_ref().into())
                    .to_string()
                    .to_ascii_uppercase()
            );
            let offset = Literal::usize_unsuffixed(offset);
            quote!(pub const #name: isize = #offset;)
        })
        .collect::<TokenStream>();

    // generate mapping from offset to name
    let register_name_map_contents = {
        let mut offset_names = rudder
            .get_registers()
            .into_iter()
            .map(|(name, (_, offset))| (offset, name))
            .collect::<Vec<_>>();

        offset_names.sort_by_key(|(offset, _)| *offset);

        offset_names
            .into_iter()
            .map(|(offset, name)| {
                let name = name.as_ref();

                let offset = Literal::isize_suffixed(isize::try_from(offset).unwrap());
                quote!((#offset, #name),)
            })
            .collect::<TokenStream>()
    };

    let registers_len = {
        let (last_register, last_offset) = rudder
            .get_registers()
            .into_values()
            .max_by_key(|(typ, offset)| offset + typ.width_bytes())
            .unwrap();

        last_offset + last_register.width_bytes()
    };

    quote! {
        // todo check this is necessary
        #[repr(align(8))]
        pub struct State {
            data: [u8; #registers_len],
            guest_memory_base: usize,
        }

        impl State {
            /// Returns the ISA state with initial values and configuration set
            pub fn init(guest_memory_base: usize) -> Self {
                let mut celf = Self {
                    data: [0; #registers_len],
                    guest_memory_base,
                };

                celf
            }

            pub fn write_register<T>(&mut self, offset: isize, value: T) {
                unsafe { *(self.data.as_ptr().byte_offset(offset) as *mut T) = value };
            }

            pub fn read_register<T: Copy>(&self, offset: isize) -> T {
                unsafe { *(self.data.as_ptr().byte_offset(offset) as *const T) }
            }

            pub fn guest_memory_base(&self) -> usize {
                self.guest_memory_base
            }
        }

        #register_offsets

        pub const REGISTER_NAME_MAP: &[(isize, &str)] = &[#register_name_map_contents];
    }
}

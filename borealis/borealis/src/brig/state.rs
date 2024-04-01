use {crate::rudder::Context, proc_macro2::TokenStream, quote::quote};

/// Code generation for ISA state
pub fn codegen_state(rudder: &Context) -> TokenStream {
    let registers_len = {
        let (last_register, last_offset) = rudder
            .get_registers()
            .into_iter()
            .max_by_key(|(typ, offset)| offset + typ.width_bytes())
            .unwrap();

        last_offset + last_register.width_bytes()
    };

    // todo: register names as constants

    quote! {
        pub use state::State;
        mod state {
            // todo check this is necessary
            #[repr(align(8))]
            pub struct State {
                data: [u8; #registers_len],
            }

            impl State {
                /// Returns the ISA state with initial values and configuration set
                pub fn init() -> Self {
                    let mut celf = Self {
                        data: [0; #registers_len]
                    };

                    // magic Tom values
                    celf.write_register(0x18ed0, [
                        false, false, false, false, true, true, true, false, false, true, true, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false,
                    ]);

                    celf
                }

                pub fn write_register<T>(&mut self, offset: isize, value: T) {
                    unsafe { *(self.data.as_ptr().byte_offset(offset) as *mut T) = value };
                }

                pub fn read_register<T: Copy>(&self, offset: isize) -> T {
                    unsafe { *(self.data.as_ptr().byte_offset(offset) as *const T) }
                }
            }
        }
    }
}

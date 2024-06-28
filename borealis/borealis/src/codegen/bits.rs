use {proc_macro2::TokenStream, quote::ToTokens};

pub type BitsValue = u128;
pub type BitsLength = u16;

pub fn codegen_bits() -> TokenStream {
    syn::parse_file(include_str!("include/bits.rs"))
        .unwrap()
        .into_token_stream()
}

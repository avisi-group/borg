use proc_macro2::Group;

use {
    proc_macro::{self, TokenStream},
    proc_macro2::{Span, TokenTree},
    quote::quote,
    syn::{
        parse::{self, Parser},
        parse_macro_input,
        punctuated::Punctuated,
        token::{Bracket, Pound},
        AttrStyle::Outer,
        Attribute, Ident, ItemStruct, Path, PathArguments, PathSegment,
    },
};

#[proc_macro_attribute]
pub fn identifiable_fromvalue(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut raw_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    // add ID field to struct
    let mut modified_struct = raw_struct.clone();
    if let syn::Fields::Named(ref mut fields) = modified_struct.fields {
        fields
            .named
            .push(syn::Field::parse_named.parse2(quote! { id: u64 }).unwrap());
    }

    // duplicate the original struct
    let original_ident = raw_struct.ident.clone();
    let raw_ident = Ident::new(&format!("Raw{}", original_ident), Span::call_site());
    raw_struct.ident = raw_ident.clone();
    raw_struct.attrs.push(derive_fromvalue_attr());

    // TODO: ensuring it derives FromValue

    // assign all fields of raw struct to modified struct
    let mut assignments = quote!();
    for field in &raw_struct.fields {
        let name = field.ident.clone().unwrap();
        assignments.extend([quote!(#name: raw.#name,)]);
    }

    return quote! {
        #modified_struct

        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #raw_struct

            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate common as _common;
            impl _common::identifiable::Identifiable for #original_ident {
                fn id(&self) -> u64 {
                    self.id
                }
            }

            unsafe impl ocaml::FromValue for #original_ident {
                fn from_value(value: ocaml::Value) -> Self {
                    let raw = #raw_ident::from_value(value);

                    Self {
                        id: _common::identifiable::unique(),
                        #assignments
                    }
                }
            }
        };
    }
    .into();
}

/// `#[derive(FromValue)]` attribute
fn derive_fromvalue_attr() -> Attribute {
    let mut segments = Punctuated::new();
    segments.push_value(PathSegment {
        ident: Ident::new("derive", Span::call_site()),
        arguments: PathArguments::None,
    });

    let tokens = TokenTree::Group(Group::new(
        proc_macro2::Delimiter::Parenthesis,
        TokenTree::Ident(Ident::new("FromValue", Span::call_site())).into(),
    ))
    .into();

    Attribute {
        pound_token: Pound {
            spans: [Span::call_site()],
        },
        style: Outer,
        bracket_token: Bracket {
            span: Span::call_site(),
        },
        path: Path {
            leading_colon: None,
            segments,
        },
        tokens,
    }
}

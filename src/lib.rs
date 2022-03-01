//! `derive_custom_enum_traits` serves as a collection of (currently one) derivable macros for dealing with enums.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

#[proc_macro_derive(EnumIndex)]
/// impls a trait for a struct that automatically converts each enum variant into an index and converts an index into an `Option` enum variant.
pub fn index_enum(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    enum_traits_inner::index_enum_inner(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

mod enum_traits_inner {
    use proc_macro2::TokenStream;
    use syn::{DeriveInput, Result};
    use quote::quote;

    pub fn index_enum_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
        let expanded = quote! {
    
        };
        Ok(TokenStream::from(expanded))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//! `derive_custom_enum_traits` serves as a collection of (currently one) derivable macros for dealing with enums.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput};

#[proc_macro_derive(DeriveIndex)]
/// impls a trait for a struct that automatically converts each enum variant into an index and converts an index into an `Option` enum variant.

/// As an example, let's take a simple enum
///
/// ```
/// pub enum Example {
///     One,
///     Two,
///     Three   
/// }
/// ```
///
/// This derive macro will automatically implement a trait called `custom_enum_traits::EnumIndex` that looks like this:
///
/// ```ignore
/// impl EnumIndex for Example {
///     fn from_index(idx: usize) -> Option<Example> {
///         match idx {
///             0 => Some(Example::One),
///             1 => Some(Example::Two),
///             2 => Some(Example::Three),
///             _ => None
///         }
///     }   
///
///     fn to_index(&self) -> usize {
///         match self {
///             Example::One => 0,
///             Example::Two => 1,
///             Example::Three => 2
///         }
///     }
/// }
///
pub fn index_enum(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    enum_traits_inner::index_enum_inner(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

mod enum_traits_inner {
    use proc_macro2::{TokenStream, Span};
    use syn::{DeriveInput, Data};
    use quote::quote;

    pub fn index_enum_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
        let variants = match &ast.data {
            Data::Enum(enum_data) => &enum_data.variants,
            _ => return Err(syn::Error::new(Span::call_site(), "this macro only supports enums"))
        };
        let mut from_idxs = Vec::new();
        let mut to_idxs = Vec::new();
        for (count, variant) in variants.iter().enumerate() {
            if !variant.fields.is_empty() {
                return Err(syn::Error::new(Span::call_site(), "this macro only works on enum variants without fields"));
            }
            from_idxs.push(quote! {#count => Some(#name::#variant)});
            to_idxs.push(quote! {#name::#variant => #count});
        }
        from_idxs.push(quote!(_ => None));
        Ok(quote!{
            #[doc = "Converts an enum to and from an index"]
            impl #impl_generics EnumIndex for #name #ty_generics #where_clause {
                fn from_index(idx: usize) -> Option<#name #ty_generics> where Self:Sized {
                    match idx {
                        #(#from_idxs),*
                    }
                }

                fn to_index(&self) -> usize {
                    match self {
                        #(#to_idxs),*
                    }
                }
            }
        })
    }
}

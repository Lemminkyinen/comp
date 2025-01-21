use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, Token,
};

struct Comprehension {
    mapping: Expr,
    var: Ident,
    sequence: Expr,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // x * 2
        let mapping: Expr = input.parse()?;

        // for
        input.parse::<Token![for]>()?;

        // x
        let var: Ident = input.parse()?;

        // in
        input.parse::<Token![in]>()?;

        // sequence
        let sequence: Expr = input.parse()?;

        Ok(Self {
            mapping,
            var,
            sequence,
        })
    }
}

/// Simple python like list comprehension
#[proc_macro]
pub fn list_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as Comprehension);

    let Comprehension {
        mapping,
        var,
        sequence,
    } = parsed;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        ::core::iter::IntoIterator::into_iter(#sequence)
            .map(|#var| #mapping)
            .collect::<Vec<_>>()
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

/// Simple python like set comprehension
#[proc_macro]
pub fn set_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as Comprehension);

    let Comprehension {
        mapping,
        var,
        sequence,
    } = parsed;
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        ::core::iter::IntoIterator::into_iter(#sequence)
            .map(|#var| #mapping)
            .collect::<::std::collections::HashSet<_>>()
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

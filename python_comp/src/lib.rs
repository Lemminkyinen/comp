use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Pat, Token,
};

struct Comprehension {
    mapping: Expr,
    for_clause: ForClause,
}

struct ForClause {
    pattern: Pat,
    sequence: Expr,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mapping = input.parse()?;
        let for_clause = input.parse()?;
        Ok(Self { mapping, for_clause })
    }
}

impl Parse for ForClause {
    /// Parse
    ///```python
    ///_ = [x for x in iter]
    ///#      ^^^^^^^^^^^^^
    ///```
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pattern = Pat::parse_single(input)?;
        input.parse::<Token![in]>()?;
        let sequence = input.parse()?;
        Ok(Self { pattern, sequence })
    }
}

/// Simple python like list comprehension
#[proc_macro]
pub fn vec_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let iter_result = iter_comp(input);
    let iter_result = proc_macro2::TokenStream::from(iter_result);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #iter_result.collect::<Vec<_>>()
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

/// Simple python like set comprehension
#[proc_macro]
pub fn set_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let iter_result = iter_comp(input);
    let iter_result = proc_macro2::TokenStream::from(iter_result);

    let expanded = quote! {
        #iter_result.collect::<::std::collections::HashSet<_>>()
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn iter_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as Comprehension);

    let Comprehension { mapping, for_clause } = parsed;
    let ForClause { pattern, sequence } = for_clause;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        ::core::iter::IntoIterator::into_iter(#sequence)
            .map(|#pattern| #mapping)
    };

    TokenStream::from(expanded)
}

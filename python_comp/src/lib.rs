use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Pat, Token,
};

struct Comprehension {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mapping = input.parse()?;
        let for_if_clause = input.parse()?;
        Ok(Self { mapping, for_if_clause })
    }
}

struct Mapping(Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

struct Pattern(Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Pat::parse_single(input).map(Self)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

struct Condition(Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

struct ForIfClause {
    pattern: Pattern,
    sequence: Expr,
    conditions: Vec<Condition>,
}

fn parse_multiple<T>(input: ParseStream) -> Vec<T>
where
    T: Parse,
{
    let mut vec = Vec::new();
    while let Ok(x) = input.parse() {
        vec.push(x);
    }
    vec
}

impl Parse for ForIfClause {
    /// Parse
    ///```python
    ///_ = [x for x in iter]
    ///#      ^^^^^^^^^^^^^
    ///```
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pattern = input.parse()?;
        input.parse::<Token![in]>()?;
        let sequence = input.parse()?;
        let conditions = parse_multiple(input);
        Ok(Self { pattern, sequence, conditions })
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

    let Comprehension { mapping, for_if_clause: for_clause } = parsed;
    let ForIfClause { pattern, sequence, conditions } = for_clause;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
    ::core::iter::IntoIterator::into_iter(#sequence)
        .filter_map(|#pattern| {
            if true #(&& (#conditions))* { Some(#mapping)}
            else { None }
        })
    };

    TokenStream::from(expanded)
}

use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Pat, Token,
};

struct Comprehension {
    mapping: Mapping,
    for_if_clause: ForIfClause,
    additional_for_if_clauses: AdditionalForIfClauses,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mapping = input.parse()?;
        let for_if_clause = input.parse()?;
        let additional_for_if_clauses = input.parse()?;
        Ok(Self { mapping, for_if_clause, additional_for_if_clauses })
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
    conditions: Conditions,
}

struct Conditions(Vec<Condition>);

impl Parse for Conditions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(parse_multiple(input)))
    }
}

fn parse_multiple<T>(input: ParseStream) -> Vec<T>
where
    T: Parse,
{
    // Calculate count of ifs
    let mut count = 0;
    let fork = input.fork();
    while let Ok(_) = fork.parse::<T>() {
        count += 1;
    }

    let mut vec = Vec::with_capacity(count);
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
        let conditions = input.parse()?;
        Ok(Self { pattern, sequence, conditions })
    }
}

struct AdditionalForIfClauses(Vec<ForIfClause>);

impl Parse for AdditionalForIfClauses {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(parse_multiple(input)))
    }
}

/// Simple python like list comprehension
#[proc_macro]
pub fn vec_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let iter_result = proc_macro2::TokenStream::from(iter_comp(input));
    quote! { #iter_result.collect::<::std::vec::Vec<_>>() }.into()
}

/// Simple python like set comprehension
#[proc_macro]
pub fn set_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let iter_result = proc_macro2::TokenStream::from(iter_comp(input));
    quote! { #iter_result.collect::<::std::collections::HashSet<_>>() }.into()
}

#[proc_macro]
pub fn iter_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as Comprehension);

    let Comprehension { mapping, for_if_clause, additional_for_if_clauses } = parsed;

    let mut for_ifs = core::iter::once(for_if_clause)
        .chain(additional_for_if_clauses.0.into_iter())
        .rev();

    let output = {
        let first_if = for_ifs.next().expect("There is always one for if clause!");
        let ForIfClause { pattern, sequence, conditions } = first_if;
        let Conditions(conditions) = conditions;
        quote! {
            ::core::iter::IntoIterator::into_iter(#sequence)
            .filter_map(|#pattern| {
                if true #(&& (#conditions))* { Some(#mapping) }
                else { None }
            })
        }
    };

    for_ifs
        .fold(output, |cur_output, next_layer| {
            let ForIfClause { pattern, sequence, conditions } = next_layer;
            let Conditions(conditions) = conditions;
            quote! {
                ::core::iter::IntoIterator::into_iter(#sequence)
                .filter_map(|#pattern| {
                    if true #(&& (#conditions))* { Some(#cur_output) }
                    else { None }
                })
                .flatten()
            }
        })
        .into()
}

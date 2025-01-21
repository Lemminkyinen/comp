use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Expr, Ident, Token};

struct ListComp {
    mapping: Expr,
    var: Ident,
    sequence: Expr,
}

fn my_parser(i: ParseStream) -> syn::Result<ListComp> {
    // x * 2
    let mapping: Expr = i.parse()?;

    // for
    i.parse::<Token![for]>()?;

    // x
    let var: Ident = i.parse()?;

    // in
    i.parse::<Token![in]>()?;

    // sequence
    let sequence: Expr = i.parse()?;

    Ok(ListComp {
        mapping,
        var,
        sequence,
    })
}

/// Simple python list comprehension
#[proc_macro]
pub fn list_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input with my_parser);

    let ListComp {
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

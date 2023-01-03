use std::ops::{Bound, RangeBounds};

use proc_macro::{Span, TokenStream};
use quote::{__private::Literal, quote};
use syn::{
    parse::Parse, parse_macro_input, Attribute, DeriveInput, Ident, Lit, Meta, MetaNameValue, Result
};

mod fast;
mod verbose;

#[proc_macro_derive(CheckVerbose, attributes(range, filter, fallback))]
pub fn derive_range_checker_verbose(input: TokenStream) -> TokenStream {
    verbose::derive_range_checker(parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Check, attributes(range, filter, fallback))]
pub fn derive_range_checker(input: TokenStream) -> TokenStream {
    fast::derive_range_checker(parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

pub(crate) fn extract_attributes(
    attrs: Vec<Attribute>,
) -> Result<(
    Vec<syn::ExprRange>,
    Vec<syn::ExprClosure>,
    Vec<syn::Lit>,
    Vec<syn::ExprClosure>,
)> {
    let mut range_attrs = vec![];
    let mut filter_attrs = vec![];
    let mut fallback_lit_attrs = vec![];
    let mut fallback_closure_attrs = vec![];

    for attr in attrs {
        match attr
            .path
            .get_ident()
            .map(|ident| ident.to_string())
            .unwrap_or_default()
            .as_str()
        {
            "range" => range_attrs.push(attr.parse_args::<syn::ExprRange>()?),
            "filter" => filter_attrs.push(attr.parse_args::<syn::ExprClosure>()?),
            "fallback" => {
                if fallback_lit_attrs.len() + fallback_closure_attrs.len() != 0 {
                    return Err(syn::Error::new(
                        Span::call_site().into(),
                        "multiple fallback attributes is not allowed",
                    ));
                }

                let lit = attr.parse_args::<syn::Lit>();
                let closure = attr.parse_args::<syn::ExprClosure>();

                if let Err(mut e_closure) = closure {
                    if let Err(e_lit) = lit {
                        e_closure.combine(e_lit);
                        return Err(e_closure);
                    } else if let Ok(lit) = lit {
                        fallback_lit_attrs.push(lit);
                    }
                } else if let Ok(closure) = closure {
                    fallback_closure_attrs.push(closure);
                }
            }
            _ => unreachable!(),
        }
    }

    Ok((
        range_attrs,
        filter_attrs,
        fallback_lit_attrs,
        fallback_closure_attrs,
    ))
}

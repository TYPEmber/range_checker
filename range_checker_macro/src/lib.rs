use std::ops::{Bound, RangeBounds};

use proc_macro::{Span, TokenStream};
use quote::{__private::Literal, quote};
use syn::{
    parse::Parse, parse_macro_input, Attribute, DeriveInput, Ident, Lit, Meta, MetaNameValue,
};

#[proc_macro_derive(RangeChecker, attributes(range, fallback))]
pub fn derive_range_checker(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let DeriveInput { ident, .. } = input;

    let mut check_list = vec![];
    let mut ident_list = vec![];
    let mut fallback_list = vec![];

    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        for field in fields {
            let ident_item = &field.ident.unwrap();
            let attrs = &field.attrs;

            let mut range_attrs = extract_attributes::<syn::ExprRange>(attrs.iter(), "range");
            let mut fallback = extract_attributes::<syn::Lit>(attrs.iter(), "fallback");

            if let Some(range_first) = range_attrs.next() {
                let mut check_statement = quote! {(#range_first).contains(&self.#ident_item)};

                let fallback = fallback
                    .next()
                    .map(|lit| quote! { Some(#lit) })
                    .unwrap_or(quote! { None });

                for range in range_attrs {
                    check_statement.extend(quote! {|| (#range).contains(&self.#ident_item)})
                }

                check_list.push(check_statement);
                ident_list.push(ident_item.clone());
                fallback_list.push(fallback);
            };
        }
    }

    // dbg!(&fallback_list);

    quote!(
        impl #ident {
            fn check(&self) -> Result<(), Vec<String>> {
                // dbg!(#(#check_list),*);

                let mut err_str = vec![];

                #(
                    if !(#check_list) {
                        err_str.push(format!(
                            "{} == false, {} = {}",
                            stringify!(#check_list),
                            stringify!(self.#ident_list),
                            self.#ident_list
                        ));
                    }
                )*

                if err_str.is_empty() {
                    Ok(())
                }else {
                    Err(err_str)
                }
            }

            fn check_with_fallback(&mut self) -> Result<Vec<String>, Vec<String>> {
                // dbg!(#(#check_list),*);
                // dbg!(#(#fallback_list)*);
                // dbg!(#(#ident_list)*);

                let mut err_str = vec![];
                let mut fallback_str = vec![];

                #(
                    if !(#check_list) {
                        // dbg!(#fallback_list);

                        let ret_str = format!(
                            "{} == false, {} = {}",
                            stringify!(#check_list),
                            stringify!(self.#ident_list),
                            self.#ident_list
                        );

                        if let Some(fallback) = #fallback_list {
                            self.#ident_list = fallback;
                            fallback_str.push(format!("{} {}", ret_str, "=> fallback success!"));
                        } else {
                            err_str.push(ret_str);
                        }
                    }
                )*

                if err_str.is_empty() {
                    Ok(fallback_str)
                }else {
                    err_str.extend(fallback_str);
                    Err(err_str)
                }
            }
        }
    )
    .into()
}

fn extract_attributes<'a, T>(
    attrs: impl Iterator<Item = &'a Attribute> + 'a,
    id_str: &'a str,
) -> impl Iterator<Item = T> + 'a
where
    T: Parse,
{
    attrs
        .map(|attr| (attr, attr.path.get_ident()))
        .filter(|(_, meta)| meta.is_some())
        .map(|(attr, ident)| (attr, ident.unwrap()))
        .filter(|(_, ident)| (*ident).eq(id_str))
        .map(|(attr, _)| attr.parse_args::<T>())
        .filter_map(|attr| attr.ok())
}

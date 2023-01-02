use std::ops::{Bound, RangeBounds};

use proc_macro::{Span, TokenStream};
use quote::{__private::Literal, quote};
use syn::{
    parse::Parse, parse_macro_input, Attribute, DeriveInput, Ident, Lit, Meta, MetaNameValue,
};

pub fn derive_range_checker(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let DeriveInput { ident, .. } = input;

    let mut check_list = vec![];
    let mut ident_list = vec![];
    let mut fallback_list = vec![];
    let mut type_list = vec![];

    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        for field in fields {
            let ident_item = &field.ident.unwrap();

            let mut range_attrs = extract_attributes::<syn::ExprRange>(field.attrs.iter(), "range");
            let mut filters = extract_attributes::<syn::ExprClosure>(field.attrs.iter(), "filter");
            let mut fallback = extract_attributes::<syn::Lit>(field.attrs.iter(), "fallback");
            let mut fallback_closure =
                extract_attributes::<syn::ExprClosure>(field.attrs.iter(), "fallback");

            // assert!(fallback.count() + fallback_closure.count() <= 1);

            let ty = field.ty.clone();
            let fallback_closure = fallback_closure
                .next()
                .map(|closure| quote! { Some(#closure) })
                .unwrap_or(
                    fallback
                        .next()
                        .map(|lit| quote! { Some(|x| #lit ) })
                        .unwrap_or(quote! { Option::<fn(#ty) -> #ty>::None }),
                );

            let mut check_statement = TokenStream::default().into();

            if let Some(range_first) = range_attrs.next() {
                check_statement = quote! {(#range_first).contains(&self.#ident_item)};

                for range in range_attrs {
                    check_statement.extend(quote! {|| (#range).contains(&self.#ident_item)});
                }
            }

            if check_statement.is_empty() {
                if let Some(filter_first) = filters.next() {
                    check_statement = quote! {(#filter_first)(self.#ident_item)};
                };
            }

            for filter in filters {
                check_statement.extend(quote! {&& (#filter)(self.#ident_item)})
            }

            if !check_statement.is_empty() {
                check_list.push(check_statement);
                ident_list.push(ident_item.clone());
                fallback_list.push(fallback_closure);
                type_list.push(ty);
            }
        }
    }

    // dbg!(&fallback_list);

    quote!(
        impl range_checker::CheckVerbose for #ident {
            fn check(&self) -> Result<(), Vec<range_checker::Error>> {
                // dbg!(#(#check_list),*);

                let mut err_vec = vec![];

                #(
                    if !(#check_list) {
                        err_vec.push(
                            range_checker::Error::CheckFailed {
                                ident: stringify!(#ident_list).to_owned(),
                                value: (self.#ident_list).to_string(),
                                check_statement: stringify!(#check_list).to_owned(),
                            }
                        )
                    }
                )*

                if err_vec.is_empty() {
                    Ok(())
                }else {
                    Err(err_vec)
                }
            }

            fn check_with_fallback(&mut self) -> Result<Vec<range_checker::Error>, Vec<range_checker::Error>> {
                // dbg!(#(#check_list),*);
                // dbg!(#(#fallback_list)*);
                // dbg!(#(#ident_list)*);

                let mut failed_vec = vec![];
                let mut fallback_vec = vec![];

                #(
                    if !(#check_list) {
                        // dbg!(#fallback_list);

                        if let Some(fallback_closure) = #fallback_list {
                            let fallback = fallback_closure(self.#ident_list);

                            fallback_vec.push(
                                range_checker::Error::Fallback {
                                    ident: stringify!(#ident_list).to_owned(),
                                    value: (self.#ident_list).to_string(),
                                    check_statement: stringify!(#check_list).to_owned(),
                                    fallback: fallback.to_string(),
                                }
                            );

                            self.#ident_list = fallback;
                        } else {
                            failed_vec.push(
                                range_checker::Error::CheckFailed {
                                    ident: stringify!(#ident_list).to_owned(),
                                    value: (self.#ident_list).to_string(),
                                    check_statement: stringify!(#check_list).to_owned(),
                                }
                            );
                        }
                    }
                )*

                if failed_vec.is_empty() {
                    Ok(fallback_vec)
                }else {
                    failed_vec.extend(fallback_vec);
                    Err(failed_vec)
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

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Result};

use super::extract_attributes;

pub fn derive_range_checker(input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput { ident, .. } = input;

    let mut check_list = vec![];
    let mut ident_list = vec![];
    let mut fallback_list = vec![];

    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        for field in fields {
            let ident_item = &field.ident.unwrap();

            let (range_attrs, filter_attrs, fallback_lit_attrs, fallback_closure_attrs) =
                extract_attributes(field.attrs)?;

            let mut check_statement = TokenStream::default();

            let mut range_attrs = range_attrs.iter();
            if let Some(range_first) = range_attrs.next() {
                check_statement = quote! {(#range_first).contains(&self.#ident_item)};

                for range in range_attrs {
                    check_statement.extend(quote! {|| (#range).contains(&self.#ident_item)});
                }
            }
            if check_statement.is_empty() {
                if let Some(filter_first) = filter_attrs.first() {
                    check_statement = quote! {(#filter_first)(&self.#ident_item)};
                };
            }

            let mut filter_attrs = filter_attrs.iter();
            if check_statement.is_empty() {
                if let Some(filter_first) = filter_attrs.next() {
                    check_statement = quote! {(#filter_first)(&self.#ident_item)};
                };
            }
            if !check_statement.is_empty() {
                let fallback_closure = fallback_closure_attrs
                    .first()
                    .map(|closure| {
                        Some(quote! {
                            let fallback = (#closure)(&self.#ident_item);
                        })
                    })
                    .unwrap_or(fallback_lit_attrs.first().map(|lit| {
                        quote! {
                            let fallback = #lit;
                        }
                    }))
                    .map(|mut tokens| {
                        tokens.extend(quote! {
                            ret_vec.push(
                                range_checker::Error::Fallback {
                                    ident: stringify!(#ident_item).to_owned(),
                                    value: (self.#ident_item).to_string(),
                                    check_statement: stringify!(#check_statement).to_owned(),
                                    fallback: fallback.to_string(),
                                }
                            );

                            self.#ident_item = fallback;
                        });
                        tokens
                    })
                    .unwrap_or(quote! {
                        failed = true;
                        ret_vec.push(
                            range_checker::Error::CheckFailed {
                                ident: stringify!(#ident_item).to_owned(),
                                value: (self.#ident_item).to_string(),
                                check_statement: stringify!(#check_statement).to_owned(),
                            }
                        );
                    });

                check_list.push(check_statement);
                ident_list.push(ident_item.clone());
                fallback_list.push(fallback_closure);
            }
        }
    } else {
        return Err(syn::Error::new(
            Span::call_site(),
            "currently only structs are supported by this derive",
        ));
    }

    Ok(quote!(
        impl range_checker::CheckVerbose for #ident {
            fn check(&self) -> Result<(), Vec<range_checker::Error>> {
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
                let mut ret_vec = vec![];
                let mut failed = false;

                #(
                    if !(#check_list) {
                        #fallback_list
                    }
                )*

                if !failed {
                    Ok(ret_vec)
                } else {
                    Err(ret_vec)
                }
            }
        }
    ))
}

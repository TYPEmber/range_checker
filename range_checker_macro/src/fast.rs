use std::ops::{Bound, RangeBounds};

use proc_macro2::{Span, TokenStream};
use quote::{__private::Literal, quote};
use syn::{
    parse::Parse, parse_macro_input, Attribute, DeriveInput, Ident, Lit, Meta, MetaNameValue,
    Result,
};

use super::extract_attributes;

pub fn derive_range_checker(input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput { ident, .. } = input;

    let mut check_list = vec![];
    let mut ident_list = vec![];
    let mut fallback_list = vec![];

    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        for field in fields {
            let ident_item = &field.ident;

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

            let mut filter_attrs = filter_attrs.iter();
            if check_statement.is_empty() {
                if let Some(filter_first) = filter_attrs.next() {
                    check_statement = quote! {(#filter_first)(&self.#ident_item)};
                };
            }
            for filter in filter_attrs {
                check_statement.extend(quote! {&& (#filter)(&self.#ident_item)})
            }

            if !check_statement.is_empty() {
                if field.ident.is_none() {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "currently only named field are supported by this derive",
                    ));
                }
                let fallback_closure = fallback_closure_attrs
                    .first()
                    .map(|closure| {
                        Some(quote! {
                            self.#ident_item = (#closure)(&self.#ident_item);
                        })
                    })
                    .unwrap_or(fallback_lit_attrs.first().map(|lit| {
                        quote! {
                            self.#ident_item = #lit;
                        }
                    }))
                    .unwrap_or(quote! {
                        return Err(());
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

    // dbg!(&fallback_list);

    Ok(quote!(
        impl ::range_checker::Check for #ident {
            fn check(&self) -> Result<(), ()> {
                // dbg!(#(#check_list),*);

                #(
                    if !(#check_list) {
                        return Err(());
                    }
                )*

                Ok(())
            }

            fn check_with_fallback(&mut self) -> Result<(), ()> {
                // dbg!(#(#check_list),*);
                // dbg!(#(#fallback_list)*);
                // dbg!(#(#ident_list)*);

                #(
                    if !(#check_list) {
                        // dbg!(#fallback_list);
                        #fallback_list
                    }
                )*

                Ok(())
            }
        }
    ))
}

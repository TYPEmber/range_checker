use std::ops::{Bound, RangeBounds};

use proc_macro::{Span, TokenStream};
use quote::{__private::Literal, quote};
use syn::{parse_macro_input, DeriveInput, Lit, Meta, MetaNameValue};

#[proc_macro_derive(RangeChecker, attributes(range, fallback))]
pub fn derive_range_checker(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let DeriveInput { ident, .. } = input;

    let mut check_list = vec![];
    let mut ident_list = vec![];
    let mut fallback_list = vec![];
    let mut ty_list = vec![];

    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        for field in fields {
            let ident_item = &field.ident.unwrap();
            let attrs = &field.attrs;

            let mut range_attrs = attrs
                .iter()
                .map(|attr| (attr, attr.path.get_ident()))
                .filter(|(_, meta)| meta.is_some())
                .map(|(attr, ident)| (attr, ident.unwrap()))
                .filter(|(_, ident)| *ident == "range")
                .map(|(attr, _)| attr.parse_args::<syn::ExprRange>())
                .filter_map(|attr| attr.ok());

            let mut fallback = attrs
                .iter()
                .map(|attr| (attr, attr.path.get_ident()))
                .filter(|(_, meta)| meta.is_some())
                .map(|(attr, ident)| (attr, ident.unwrap()))
                .filter(|(_, ident)| *ident == "fallback")
                .map(|(attr, _)| attr.parse_args::<syn::Lit>().unwrap())
                // .map(|lit| lit)
                .collect::<Vec<_>>();

            // if (..5).start_bound() != Bound::Unbounded {}

            // if let Bound::Included(start) = (..5).start_bound() {

            // } else if let Bound::Included(end) = (..5).end_bound() {

            // }     

            if let Some(range_first) = range_attrs.next() {
                let mut check_statement = quote! {(#range_first).contains(&self.#ident_item)};

                let ty = field.ty.clone();
                let fallback = fallback
                    .pop()
                    .map(|lit| quote! {#lit})
                    // .unwrap_or(quote! {(#range_first).next().unwrap()});
                    .unwrap_or(quote! {
                        // if (#range_first).contains(&<(#ty) as Default>::default()) {
                        //     <(#ty) as Default>::default()
                        // } else {
                        //     <(#ty) as Default>::default() 
                        // }

                        <(#ty) as Default>::default()
                    });

                for range in range_attrs {
                    check_statement.extend(quote! {|| (#range).contains(&self.#ident_item)})
                }

                check_list.push(check_statement);
                ident_list.push(ident_item.clone());
                fallback_list.push(fallback);
                ty_list.push(field.ty);
            };
        }
    }

    dbg!(&fallback_list);

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

            fn check_with_fallback(&mut self) -> Result<(), Vec<String>> {
                // dbg!(#(#check_list),*);
                // dbg!(#(#fallback_list)*);
                // dbg!(#(#ident_list)*);

                let mut err_str = vec![];

                #(
                    if !(#check_list) {
                        // dbg!(#fallback_list);
                        err_str.push(format!(
                            "{} == false, {} = {}",
                            stringify!(#check_list),
                            stringify!(self.#ident_list),
                            self.#ident_list
                        ));

                        self.#ident_list = #fallback_list;
                    }
                )*

                if err_str.is_empty() {
                    Ok(())
                }else {
                    Err(err_str)
                }
            }
        }
    )
    .into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RangeChecker, attributes(range))]
pub fn derive_range_checker(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let DeriveInput { ident, .. } = input;

    let mut check_list = vec![];
    let mut ident_list = vec![];

    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        for field in fields {
            let ident_item = &field.ident.unwrap();
            let attrs = &field.attrs;

            if !attrs.is_empty() {
                if let Ok(range) = attrs[0].parse_args::<syn::ExprRange>() {
                    check_list.push(quote! {(#range).contains(&self.#ident_item)});
                    ident_list.push(quote! {&self.#ident_item});
                }
            }
        }
    }

    quote!(
        impl #ident {
            fn check(&self) -> Result<(), Vec<String>> {
                // dbg!(#(#check_list),*);

                let mut err_str = vec![];

                #(
                    if !#check_list {
                        err_str.push(format!(
                            "{} == false, {} = {}",
                            stringify!(#check_list),
                            stringify!(#ident_list),
                            #ident_list
                        ));
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

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn hidstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let key: u8 = 42;

    let obfuscated: Vec<u8> = input.value().bytes().map(|b| b ^ key).collect();
    let obfuscated_lit = obfuscated.iter().map(|b| quote! { #b, });

    TokenStream::from(quote! {
        {
            let obfuscated = [#(#obfuscated_lit)*];
            obfuscated.iter().map(|&b| (b ^ #key) as char).collect()
        }
    })
}

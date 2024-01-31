use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
pub fn group(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let mut iter = input.into_iter();

    let next_token = match iter.next() {
        Some(token) => token,
        None => panic!("unexpected end of input"),
    };
    let ident = match next_token {
        TokenTree::Ident(ident) => {
            let ident = Ident::new(&format!("__dwnorg_{}", ident), Span::call_site());
            ident
        }
        _ => panic!("expected an identifier"),
    };

    let next_token = match iter.next() {
        Some(token) => token,
        None => panic!("unexpected end of input"),
    };
    match next_token {
        TokenTree::Punct(punct) => {
            if punct.as_char() == ';' {
            } else {
                panic!("expected a semicolon")
            }
        }
        _ => panic!("expected a semicolon"),
    };

    let next_token = match iter.next() {
        Some(token) => token,
        None => panic!("unexpected end of input"),
    };
    let exts = match next_token {
        TokenTree::Group(group) => group,
        _ => panic!("expected a group"),
    };

    quote!(
        #[deny(unused)]
        pub fn #ident() {}
    )
    .into()
}

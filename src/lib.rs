use litrs::StringLit;
use proc_macro::{Literal, TokenStream, TokenTree};
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::quote;
use std::ffi::CString;

#[proc_macro]
pub fn c(token_stream: TokenStream) -> TokenStream {
    let tokens: Vec<_> = token_stream.into_iter().collect();
    assert!(tokens.len() == 1, "Macro argument must be a single token");

    let str_lit = StringLit::try_from(&tokens[0]).expect("Failed to transform token to C literal");
    let cstr = CString::new(str_lit.value()).expect("String literal must not contain null bytes");
    let cstr_lit = TokenTree::Literal(Literal::c_string(cstr.as_c_str())).into_token_tree2();

    quote! { #cstr_lit.as_ptr() }.into()
}

trait TokenTreeExt {
    fn into_token_tree2(self) -> TokenTree2;
}

impl TokenTreeExt for TokenTree {
    fn into_token_tree2(self) -> TokenTree2 {
        // Yes, this is dumb.
        let stream: TokenStream = self.into();
        let stream2: TokenStream2 = stream.into();
        stream2.into_iter().next().expect("Expected one token")
    }
}

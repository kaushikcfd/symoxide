use proc_macro;
use lazy_static::lazy_static;
use proc_macro::{TokenStream};
use syn::{parse, LitStr, LitInt, LitFloat, Result, Expr, parse_str};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use regex::Regex;
use quote:: quote;


lazy_static! {
    static ref RE_SENTENCE: Regex = Regex::new(
        r"^(([a-zA-Z_][_0-9a-zA-Z]*)\s+)*([a-zA-Z_][_0-9a-zA-Z]*)\s*$"
        ).unwrap(); 

    static ref RE_WORD: Regex = Regex::new(
        r"(?P<id>[a-zA-Z_][_0-9a-zA-Z]*)"
        ).unwrap();
}


fn get_words(s: &str) -> Vec<&str> {
    if ! RE_SENTENCE.is_match(s) {
        panic!("Provided sentence does not correspond to a collection of words");
    }

    RE_WORD
        .captures_iter(s)
        .map(|capture| capture.name("id").unwrap().as_str())
        .collect()
}

fn parse_variables_string_stream(s: String) -> TokenStream {
    let words = get_words(&s[..]);
    let mut word_list: Punctuated<Expr,Comma> = Punctuated::new();
    for word in words {
        let str_to_parse = format!("symoxide::var(\"{}\")", word);
        // TODO: Avoid this call to `parse_str`
        word_list.push(parse_str(&str_to_parse[..]).unwrap());
    }
    let gen = quote! {(#word_list,)};
    gen.into()
}

#[proc_macro]
pub fn variables(token_stream: TokenStream) -> TokenStream {
    let item: Result<LitStr> = parse(token_stream);
    match item {
        Ok(x) => {parse_variables_string_stream(x.value())},
        Err(_) => {panic!("split! expects a string literal.")}
    }
}


#[proc_macro]
pub fn scalar(token_stream: TokenStream) -> TokenStream {
    let item: Result<LitInt> = parse(token_stream.clone());
    match item {
        Ok(x) => {
            let gen = quote! { std::rc::Rc::new(symoxide::Expression::Scalar(symoxide::ScalarT::I32(#x))) };
            gen.into()
        }
        Err(_) => {
            let item: Result<LitFloat> = parse(token_stream);
            match item {
                Ok(x) => {
                    let gen = quote! { std::rc::Rc::new(symoxide::Expression::Scalar(symoxide::ScalarT::F64(#x))) };
                    gen.into()
                }
                Err(_) => panic!("split! expects a int/float literal.")
            }
        }
    }
}

// vim: fdm=marker

use proc_macro;
use lazy_static::lazy_static;
use proc_macro::{TokenStream};
use syn::{parse, LitStr, LitInt, LitFloat, Result, Expr, parse_str,
          DeriveInput, Data, Fields, Type, PathArguments, parse_macro_input};
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


#[proc_macro_derive(CachedMapper)]
pub fn derive_cached_mapper(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let struct_ = match data {
        Data::Struct(x) => x,
        _ => panic!("expected a struct"),
    };
    let struct_fields = match struct_.fields {
        Fields::Named(x) => x,
         _ => panic!("derive CachedMapper expects structs with named fields."),
    };

    let val_field_opt = struct_fields.named
        .iter()
        .filter(|x| match &x.ident {
                Some(x) => x.to_string() =="cache",
                _ => false
            }).next();
    let val_field = match val_field_opt {
        Some(x) => x,
        None => panic!("derive CachedMapper requires a 'cache' hashmap field."),
    };
    let ty = val_field.clone().ty;
    let path = match ty {
        Type::Path(ty_path) => ty_path.path,
         _ => panic!("derive CachedMapper requires 'cache' hashmap field."),
    };

    let path_arguments = match path.segments.last() {
        Some(x) => x.clone().arguments,
         _ => panic!("derive CachedMapper requires 'cache' hashmap field."),
    };

     let hashmap_args = match &path_arguments {
         PathArguments::AngleBracketed(args) => args.clone().args,
         _ => panic!("derive CachedMapper requires 'cache' hashmap field."),
     };

    if hashmap_args.len() != 2 {
        panic!("derive CachedMapper requires 'cache' hashmap field.");
    }

    let kt = hashmap_args.first().unwrap();
    let vt = hashmap_args.last().unwrap();

    let output = quote! {
        impl CachedMapper<#kt, #vt> for #ident {
            fn query_cache(&self, key: &#kt) -> Option<&#vt> {
                self.cache.get(key)
            }
            fn add_to_cache(&mut self, key: #kt, val: #vt) {
                self.cache.insert(key, val);
            }
        }
    };
    output.into()
}

// vim: fdm=marker

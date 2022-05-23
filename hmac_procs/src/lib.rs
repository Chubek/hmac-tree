extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate lazy_static;

use std::{str::FromStr, thread::panicking};

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{*, spanned::Spanned};
use proc_macro2;
use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;


#[proc_macro_attribute]
pub fn htree_json(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);

    let ignore = args
                            .to_string()
                            .split("=")
                            .nth(1)
                            .unwrap()
                            .replace("]", "")
                            .replace("[", "")
                            .split(",")
                            .map(|x| x.trim()
                            .to_ascii_lowercase())
                            .collect::<Vec<String>>();

    let name = &item.ident;
    let fields = &item.fields;

    let mut stream_str = format!(r#"
            impl hmac_tree::encoders::serializer::HtreeJsonSerializer for {} {{
                fn ser_into_json(&self) -> Vec<u8> {{
                    

    "#, quote!(#name).to_string().replace("\"", ""));

    let mut json_str = r##"
            let json_ret = format!(r#"{{    
    "##.to_string();

    let mut format_args = String::new();
   
    let expanded = {
        for (i, field) in fields.into_iter().enumerate() {           
            let fname = match &field.ident {
                Some(sname) => quote!(#sname).to_string().replace("\"", ""),
                None => "NO NAME".to_string(),
            };

            if ignore.contains(&fname.to_lowercase()) {
                continue;
            }

            let ftype_str = match &field.ty {
                Type::Array(t) => quote!(#t).to_string(),
                Type::BareFn(t) => quote!(#t).to_string(),
                Type::Group(t) => quote!(#t).to_string(),
                Type::ImplTrait(t) => quote!(#t).to_string(),
                Type::Infer(t) => quote!(#t).to_string(),
                Type::Macro(t) => quote!(#t).to_string(),
                Type::Never(t) => quote!(#t).to_string(),
                Type::Paren(t) => quote!(#t).to_string(),
                Type::Path(t) => quote!(#t).to_string(),
                Type::Ptr(t) => quote!(#t).to_string(),
                Type::Reference(t) => quote!(#t).to_string(),
                Type::Slice(t) => quote!(#t).to_string(),
                Type::TraitObject(t) => quote!(#t).to_string(),
                Type::Tuple(t) => quote!(#t).to_string(),
                Type::Verbatim(t) => quote!(#t).to_string(),
                _ => "".to_string(),
            };
            
            if ftype_str != "Vec < u8 >" {
                panic!("Type must be byte vector (Vec<u8>)")
            }

            stream_str = format!(r#"
                    {}
                    let {}_getter = hmac_tree::encoders::serializer::htree_vec_serializer(&self.{});
                    let {}_len = self.{}.len();
            "#, stream_str, fname, fname, fname, fname);

            json_str = format!(r#"
                    {}
                    "field_{}": {{{{
                        "name": "{}",
                        "type": "{}",
                        "value": "{{{}_getter}}",
                        "len": {{{}_len}},
                    }}}},

            "#, json_str, &i, fname, ftype_str, fname, fname);

            format_args = format!("{format_args} {fname}_getter = {fname}_getter, {fname}_len = {fname}_len,  ")

        }

        json_str = format!("{json_str} \"object_name\": \"{{object_name}}\" ");

        format_args = format!("{format_args} object_name = \"{name}\"");


        json_str = format!("{json_str} }}}}\"#, {format_args});");
        stream_str = format!(r#"
                    {stream_str}

                    {json_str}

                    let ret_vec = json_ret.as_bytes().to_vec();

                    ret_vec

                }}
            }}
        
        "#);

        
        let ts = proc_macro2::TokenStream::from_str(stream_str.as_str()).unwrap();
        
        quote! {
            #item

            #ts            
        }
    };


    expanded.into()
}


use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::parse2;

fn main() {
    let t = TokenStream::from_str("{ println!(\"sss\") }").unwrap();
    let p = parse2::<syn::Expr>(t).unwrap();

    if let syn::Expr::Block(b) = p {
        b.
    }
}

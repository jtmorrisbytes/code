use std::str::FromStr;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntoTemplate)]
pub fn derive_into_template(input: TokenStream) -> TokenStream {
    let input_2 = input.clone();
    let derive_input = parse_macro_input!(input_2 as DeriveInput);
    // println!("{}",derive_input.ident);

    let output = format!(
        r#"impl crate::web_server::response::IntoTemplate for {0} where Self: serde::Serialize {{ 
               fn into_template(self) -> rocket_dyn_templates::Template {{
                    rocket_dyn_templates::Template::render("{0}",self)
               }}}}"#,
        derive_input.ident
    );

    match TokenStream::from_str(&output) {
        Ok(t) => t,
        Err(e) => TokenStream::from_str(&format!(r#"compile_error("{}")"#, e.to_string())).unwrap(),
    }
}

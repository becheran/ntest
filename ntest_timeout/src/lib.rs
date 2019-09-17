//! Part of the ntest library. Add timeout attribute to the rust test framework.

extern crate proc_macro;
extern crate syn;
extern crate timebomb;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// TODO
/// ```
#[proc_macro_attribute]
pub fn timeout(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let attribute_args = parse_macro_input!(attr as syn::AttributeArgs);
    let name = &input.sig.ident;
    let body = &input.block;
    let timeout_ms = get_timeout(&attribute_args);
    
    let result = quote! {
        fn #name() {
            timebomb::timeout_ms(|| {
            #body
            }, #timeout_ms);
         }
    };
    result.into()
}

fn get_timeout(attribute_args: &syn::AttributeArgs) -> u32 {
    if attribute_args.len() > 1 {
        panic!("Only one integer expected. Example: #[timeout(10)]");
    }
    match &attribute_args[0] {
        syn::NestedMeta::Meta(_) => {panic!("Integer expected. Example: #[timeout(10)]");},
        syn::NestedMeta::Lit(lit) => {
            match lit {
                syn::Lit::Int(int) => {return int.base10_parse::<u32>().expect("Integer expected")}
                _ => { panic!("Integer as timeout in ms expected. Example: #[timeout(10)]");}
            }
        }
    }
}

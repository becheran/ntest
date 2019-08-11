extern crate proc_macro;
extern crate syn;
use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.ident;
    let inputs = &input.decl.inputs;
    //println!("item: \"{}\"", inputs[0].to_string());

    // Our input function is always equivalent to returning 42, right?
    let result = quote! {
        #[test]
        fn #name() {
            assert!(true);
        }
    };
    result.into()
}

/*
#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_string = get_attr_string(&attr);
    let input_string = format!("#[test_case{}]{}", attr_string, input);
    let ast          = syn::parse_token_trees(&input_string);

    match ast {
        Ok(token_tree) => {
            let test_case_suit : TestCaseSuit = token_tree.into();
            let test_cases =
                test_case_suit
                    .gen_test_cases()
                    .to_string();

            TokenStream::from_str(&test_cases)
                .expect(&format!("generate test cases for: {}", input_string))
        },
        Err(e) => panic!(e)
    }
}

fn get_attr_string(attr: &TokenStream) -> String {
    let result = format!("{}", attr);

    if result.starts_with("(") {
        result
    }
    else {
        format!("({})", result)
    }
}
*/
extern crate proc_macro;
extern crate syn;

use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    let attr = parse_macro_input!(attr as syn::AttributeArgs);
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let test_case_settings = parse_test_case_attributes(&attr);

    let test_case_name = syn::Ident::new(
        &format!("{}{}", &input.ident.to_string(), &test_case_settings.name),
        Span::call_site());

    let inputs = &input.decl.inputs;

    let result = quote! {
        #[test]
        fn #test_case_name() {
            assert!(true);
        }
    };
    result.into()
}


struct TestCaseAttributes {
    literals: Vec<syn::Lit>,
    name: String,
    // TODO add Meta attributes test_name and expected_result
}

fn parse_test_case_attributes(attr: &syn::AttributeArgs) -> TestCaseAttributes {
    let mut literals: Vec<syn::Lit> = vec![];
    let mut name = "".to_string();;
    for a in attr {
        match a {
            syn::NestedMeta::Meta(m) => println!("meta"),
            syn::NestedMeta::Literal(lit) => {
                literals.push((*lit).clone());
                name.push_str(&format!("_{}",lit_to_str(lit)));
            },
        }
    }
    println!("{}", name);

    TestCaseAttributes {
        literals,
        name,
    }
}

fn lit_to_str(lit: &syn::Lit) -> String{
    match lit {
        syn::Lit::Bool(s) => s.value.to_string(),
        syn::Lit::Str(s) => s.value().to_string(),
        syn::Lit::Int(s) => s.value().to_string(),
        _ => unimplemented!(),
    }
}
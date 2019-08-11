extern crate proc_macro;
extern crate syn;

use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;
use quote::quote;
use syn::token::Token;
use syn::export::TokenStream2;

#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    let mut test_case_descriptions: Vec<TestCaseDescription> = vec![];
    let attr = parse_macro_input!(attr as syn::AttributeArgs);
    let input = parse_macro_input!(item as syn::ItemFn);

    // Collect test case descriptions
    test_case_descriptions.push(parse_test_case_attributes(&attr));
    for at in &input.attrs {
        let meta = at.parse_meta();
        match meta {
            Ok(m) => {
                match m {
                    syn::Meta::List(ml) => {
                        if ml.ident != "test_case" {
                            panic!("Only test_case attributes expected, but found {:?}", ml.ident);
                        }
                        let argument_args : syn::AttributeArgs = ml.nested.into_iter().collect();
                        test_case_descriptions.push(parse_test_case_attributes(&argument_args));
                    }
                    syn::Meta::Word(i) => {
                        panic!("Wrong input {:?} for test cases", i)
                    }
                    syn::Meta::NameValue(l) => {
                        // TODO check for named values
                        unimplemented!()
                    }
                }
            }
            Err(e) => panic!("Could not determine meta data. Error {}", e)
        }
    }

    let test_double: Vec<String> = vec!["hello".to_string(), "world".to_string()];

    let test_case_name = syn::Ident::new(
        &format!("{}{}", &input.ident.to_string(), &test_case_descriptions[0].name),
        Span::call_site());


    let function_arguments = &input.decl.inputs;

    let mut result = quote! {
        #[test]
        fn #test_case_name() {
            #(println!(#test_double);)*
            assert!(true);
        }
    };
    result.into()
}


struct TestCaseDescription {
    literals: Vec<syn::Lit>,
    name: String,
    // TODO add Meta attributes expected_result
}

fn parse_test_case_attributes(attr: &syn::AttributeArgs) -> TestCaseDescription {
    let mut literals: Vec<syn::Lit> = vec![];
    let mut name = "".to_string();

    for a in attr {
        match a {
            syn::NestedMeta::Meta(m) => println!("meta"),
            syn::NestedMeta::Literal(lit) => {
                literals.push((*lit).clone());
                name.push_str(&format!("_{}", lit_to_str(lit)));
            }
        }
    }
    println!("{}", name);

    TestCaseDescription {
        literals,
        name,
    }
}

fn lit_to_str(lit: &syn::Lit) -> String {
    match lit {
        syn::Lit::Bool(s) => s.value.to_string(),
        syn::Lit::Str(s) => s.value().to_string(),
        syn::Lit::Int(s) => s.value().to_string(),
        _ => unimplemented!(),
    }
}
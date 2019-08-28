//! Part of the ntest library. Add test cases to the rust test framework.

extern crate proc_macro;
extern crate syn;

use proc_macro2::Span;
use syn::parse_macro_input;
use proc_macro::TokenStream;
use quote::quote;
use syn::export::TokenStream2;


/// Test cases can be used to have multiple inputs for a given function. 
/// With the *test_case* attribute multiple tests will be generated using the 
/// [Procedural Macros](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html) 
/// capabilities of rust.
/// 
/// # Examples
/// 
/// Example with a single argument
/// ```ignore
/// #[test_case(13)]
/// #[test_case(42)]
/// fn one_arg(x: u32) {
///     assert!(x == 13 || x == 42)
/// }
/// ```
///
/// Example with multiple arguments:
/// ```ignore
/// #[test_case(13, 13)]
/// #[test_case(42, 42)]
/// fn two_args(x: u32, y: u32) {
///     assert_eq!(x, y);
/// }

///```
#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, item: TokenStream) -> TokenStream {
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
                        let argument_args: syn::AttributeArgs = ml.nested.into_iter().collect();
                        test_case_descriptions.push(parse_test_case_attributes(&argument_args));
                    }
                    syn::Meta::Word(i) => {
                        panic!("Wrong input {:?} for test cases", i)
                    }
                    syn::Meta::NameValue(_) => {
                        unimplemented!("Need to check for named values");
                    }
                }
            }
            Err(e) => panic!("Could not determine meta data. Error {}", e)
        }
    }

    let fn_args = &input.decl.inputs;
    let fn_body = &input.block;
    let mut fn_args_idents: Vec<syn::Ident> = vec![];

    for i in fn_args {
        match i {
            syn::FnArg::Captured(c) => {
                match &c.pat {
                    syn::Pat::Ident(ident) => {
                        fn_args_idents.push(ident.ident.clone());
                    }
                    _ => panic!("Unexpected function identifier.")
                }
            }
            _ => panic!("Unexpected function identifier.")
        }
    }

    let mut result = TokenStream2::new();
    for test_case_description in test_case_descriptions {
        let test_case_name = syn::Ident::new(
            &format!("{}{}", &input.ident.to_string(), &test_case_description.name),
            Span::call_site(),
        );
        let literals = test_case_description.literals;
        if &literals.len() != &fn_args_idents.len() {
            panic!("Test case arguments and function input signature do not match");
        }

        // Needs to be immutable
        let fn_args_idents = fn_args_idents.clone();

        let test_case_quote = quote! {
            #[test]
            fn #test_case_name() {
                let x = 42;
                #(let #fn_args_idents = #literals;)*
                #fn_body
            }
        };
        result.extend(test_case_quote);
    }
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
            syn::NestedMeta::Meta(_) => {
                unimplemented!("Need to check for named values");
            }
            syn::NestedMeta::Literal(lit) => {
                literals.push((*lit).clone());
                name.push_str(&format!("_{}", lit_to_str(lit)));
            }
        }
    }

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
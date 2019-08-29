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
    let attributes_first_test_case = parse_macro_input!(attr as syn::AttributeArgs);
    let input = parse_macro_input!(item as syn::ItemFn);

    // Collect test case descriptions
    test_case_descriptions.push(parse_test_case_attributes(&attributes_first_test_case));
    for attribute in &input.attrs {
        let meta = attribute.parse_meta();
        match meta {
            Ok(m) => {
                match m {
                    syn::Meta::Path(p) => {
                        let identifier = p.get_ident().expect("Expected identifier!");
                        if identifier != "test_case" {
                            panic!("Only test_case attributes expected, but found {:?}.", identifier);
                        }
                    }
                    syn::Meta::List(ml) => {
                        let argument_args: syn::AttributeArgs = ml.nested.into_iter().collect();
                        test_case_descriptions.push(parse_test_case_attributes(&argument_args));
                    }
                    syn::Meta::NameValue(_) => {
                        unimplemented!("Named values currently not supported.");
                    }
                }
            }
            Err(e) => panic!("Could not determine meta data. Error {}.", e)
        }
    }

    let fn_args = &input.sig.inputs;
    let fn_body = &input.block;
    let mut fn_args_idents: Vec<syn::Ident> = vec![];

    for i in fn_args {
        match i {
            syn::FnArg::Typed(t) => {
                let ubox_t = *(t.pat.clone());
                match ubox_t {
                    syn::Pat::Ident(i) => {
                        fn_args_idents.push(i.ident.clone());
                    }
                    _ => panic!("Unexpected function identifier.")
                }
            }
            syn::FnArg::Receiver(_) => panic!("Receiver function not expected for test case attribute.")
        }
    }

    let mut result = TokenStream2::new();
    for test_case_description in test_case_descriptions {
        let test_case_name = syn::Ident::new(
            &format!("{}{}", &input.sig.ident, &test_case_description.name),
            Span::call_site(),
        );
        let literals = test_case_description.literals;
        if &literals.len() != &fn_args_idents.len() {
            panic!("Test case arguments and function input signature mismatch.");
        }

        // Needs to be immutable
        let fn_args_idents = fn_args_idents.clone();

        let test_case_quote = quote! {
            #[test]
            fn #test_case_name() {
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
                panic!("Unknown test case input type.");
            }
            syn::NestedMeta::Lit(lit) => {
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
        syn::Lit::Int(s) => s.base10_digits().to_string(),
        syn::Lit::Float(s) => s.base10_digits().to_string().replace(".", "d"),
        _ => unimplemented!("String conversion for literal."),
    }
}
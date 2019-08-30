//! Part of the ntest library. Add test cases to the rust test framework.

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::export::TokenStream2;
use syn::parse_macro_input;

/// Test cases can be used to have multiple inputs for a given function.
/// With the *test_case* attribute multiple tests will be generated using the
/// [Procedural Macros](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html)
/// capabilities of rust.
///
/// The function input can be of type `int`, `bool`, or `str`.
///
/// WARNING!
/// It is currently not possible to have negative numbers as macro input. For example
/// this `#[test_case(-13)]` will not work.
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
/// #[test_case(true, "true", 1)]
/// fn test_mix(x: bool, y: &str, z: u16) {
///     assert!(x);
///     assert_eq!(y, "true");
///     assert_eq!(z, 1);
/// }
///```
#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut test_case_descriptions: Vec<TestCaseDescription> = vec![];
    let input = parse_macro_input!(item as syn::ItemFn);

    // Collect test case descriptions
    let attributes_first_test_case = parse_macro_input!(attr as syn::AttributeArgs);
    test_case_descriptions.push(parse_test_case_attributes(&attributes_first_test_case));
    for attribute in &input.attrs {
        let meta = attribute.parse_meta();
        match meta {
            Ok(m) => match m {
                syn::Meta::Path(p) => {
                    let identifier = p.get_ident().expect("Expected identifier!");
                    if identifier != "test_case" {
                        panic!(
                            "Only test_case attributes expected, but found {:?}.",
                            identifier
                        );
                    }
                }
                syn::Meta::List(ml) => {
                    let argument_args: syn::AttributeArgs = ml.nested.into_iter().collect();
                    test_case_descriptions.push(parse_test_case_attributes(&argument_args));
                }
                syn::Meta::NameValue(_) => {
                    unimplemented!("Named values currently not supported.");
                }
            },
            Err(e) => panic!("Could not determine meta data. Error {}.", e),
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
                    _ => panic!("Unexpected function identifier."),
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("Receiver function not expected for test case attribute.")
            }
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

    TestCaseDescription { literals, name }
}

fn lit_to_str(lit: &syn::Lit) -> String {
    match lit {
        syn::Lit::Bool(s) => s.value.to_string(),
        syn::Lit::Str(s) => string_to_identifier(&s.value()),
        syn::Lit::Int(s) => number_to_identifier(s.base10_digits()),
        syn::Lit::Float(s) => number_to_identifier(s.base10_digits()),
        _ => unimplemented!("String conversion for literal. Only bool, str, positive int, and float values are supported."),
    }
}

fn number_to_identifier(num: &str) -> String {
    num.chars()
        .map(|x| match x {
            '.' => 'd',
            '0'...'9' => x,
            _ => panic!("This is not a valid number. Contains unknown sign {}", x),
        })
        .collect()
}

fn string_to_identifier(num: &str) -> String {
    num.chars()
        .map(|x| match x {
            '0'...'9' => x,
            'a'...'z' => x,
            'A'...'Z' => x,
            _ => '_',
        })
        .collect()
}

//! Part of the ntest library. Add test cases to the rust test framework.

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;
mod syn_helper;

/// Test cases can be used to have multiple inputs for a given function.
/// With the `#[test_case]` attribute multiple tests will be generated using the
/// [Procedural Macros](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html)
/// capabilities of rust.
///
/// The function input can be of type `int`, `bool`, or `str`.
///
/// Please note that rust functions can only contain alphanumeric characters and '_' signs.
/// Special characters will be escaped using a meaning full replacement (for example `#` will be replaced with `_hash`),
/// or as a default the '_' sign.
///
/// WARNING!
/// It is currently not possible to have negative numbers as macro input. For example
/// this `#[test_case(-13)]` will not work.
///
/// A function annoteded with a `#[test_case]` attribute will be split into multiple rust functions annoteded with the `#[test]` attribute.
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
/// The test cases above will be parsed at compile time and two rust test functions will be generated instead:
/// ```ignore
/// #[test]
/// fn one_arg_13() {
///     x = 13;
///     assert!(x == 13 || x == 42)
/// }
///
/// #[test]
/// fn one_arg_42() {
///     x = 42;
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
/// ```
///
/// Example with name attribute:
/// ```ignore
/// #[test_case(42, name="my_fancy_test")]
/// fn with_name(x: u32) {
///     assert_eq!(x, 42)
/// }
/// ```
///
/// Example with rust test attributes.
/// All attributes after a test case will be appended after the generated `#[test]` attribute.
/// For example the following test cases...
///
/// ```ignore
/// #[test_case(18)]
/// #[ignore]
/// #[test_case(15)]
/// #[should_panic(expected = "I am panicing")]
/// fn attributes_test_case(x: u32) {
///     panic!("I am panicing");
/// }
/// ```
///
/// ... will be compiled to these two tests. One gets ignored and the other suceeds:
///
///  ```ignore
/// #[test]
/// #[ignore]
/// fn attributes_test_case_18 {
///     let x = 18;
///     panic!("I am panicing");
/// }
///
/// #[test]
/// #[should_panic(expected = "I am panicing")]
/// fn attributes_test_case_15() {
///     let x = 15;
///     panic!("I am panicing");
/// }
/// ```
#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let attribute_args = parse_macro_input!(attr as syn::AttributeArgs);

    let test_descriptions: Vec<TestDescription> =
        collect_test_descriptions(&input, &attribute_args);
    let fn_body = &input.block;
    let fn_args_idents = collect_function_arg_idents(&input);

    let mut result = proc_macro2::TokenStream::new();
    for test_description in test_descriptions {
        let test_case_name = syn::Ident::new(&test_description.name, Span::call_site());
        let literals = test_description.literals;
        let attributes = test_description.attributes;
        if &literals.len() != &fn_args_idents.len() {
            panic!("Test case arguments and function input signature mismatch.");
        }

        let test_case_quote = quote! {
            #[test]
            #(#attributes)*
            fn #test_case_name() {
                #(let #fn_args_idents = #literals;)*
                #fn_body
            }
        };
        result.extend(test_case_quote);
    }
    result.into()
}

fn collect_function_arg_idents(input: &syn::ItemFn) -> Vec<syn::Ident> {
    let mut fn_args_idents: Vec<syn::Ident> = vec![];
    let fn_args = &input.sig.inputs;
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
    fn_args_idents
}

struct TestDescription {
    literals: Vec<syn::Lit>,
    name: String,
    attributes: Vec<syn::Attribute>,
}

fn collect_test_descriptions(
    input: &syn::ItemFn,
    attribute_args: &syn::AttributeArgs,
) -> Vec<TestDescription> {
    let mut test_case_descriptions: Vec<TestDescription> = vec![];

    let fn_name = input.sig.ident.to_string();
    let test_case_parameter = parse_test_case_attributes(&attribute_args);
    let test_name = calculate_test_name(&test_case_parameter, &fn_name);
    let curr_test_attributes = TestDescription {
        literals: test_case_parameter.literals,
        name: test_name,
        attributes: vec![],
    };
    test_case_descriptions.push(curr_test_attributes);
    for attribute in &input.attrs {
        let meta = attribute.parse_meta();
        match meta {
            Ok(m) => match m {
                syn::Meta::Path(p) => {
                    let identifier = p.get_ident().expect("Expected identifier!");
                    if identifier == "test_case" {
                        panic!("Test case attributes need at least one argument such as #[test_case(42)].");
                    } else {
                        test_case_descriptions
                            .last_mut()
                            .unwrap()
                            .attributes
                            .push(attribute.clone());
                    }
                }
                syn::Meta::List(ml) => {
                    let identifier = ml.path.get_ident().expect("Expected identifier!");
                    if identifier == "test_case" {
                        let argument_args: syn::AttributeArgs = ml.nested.into_iter().collect();
                        let test_case_parameter = parse_test_case_attributes(&argument_args);
                        let test_name = calculate_test_name(&test_case_parameter, &fn_name);
                        let curr_test_attributes = TestDescription {
                            literals: test_case_parameter.literals,
                            name: test_name,
                            attributes: vec![],
                        };
                        test_case_descriptions.push(curr_test_attributes);
                    } else {
                        test_case_descriptions
                            .last_mut()
                            .unwrap()
                            .attributes
                            .push(attribute.clone());
                    }
                }
                syn::Meta::NameValue(_) => {
                    test_case_descriptions
                        .last_mut()
                        .unwrap()
                        .attributes
                        .push(attribute.clone());
                }
            },
            Err(e) => panic!("Could not determine meta data. Error {}.", e),
        }
    }
    test_case_descriptions
}

struct TestCaseAttributes {
    literals: Vec<syn::Lit>,
    custom_name: Option<String>,
}

fn parse_test_case_attributes(attr: &syn::AttributeArgs) -> TestCaseAttributes {
    let mut literals: Vec<syn::Lit> = vec![];
    let mut custom_name: Option<String> = None;

    for a in attr {
        match a {
            syn::NestedMeta::Meta(m) => match m {
                syn::Meta::Path(_) => {
                    panic!("Path not expected.");
                }
                syn::Meta::List(_) => {
                    panic!("Metalist not expected.");
                }
                syn::Meta::NameValue(nv) => {
                    let identifier = nv.path.get_ident().expect("Expected identifier!");
                    if identifier == "test_name" || identifier == "name" {
                        if custom_name.is_some() {
                            panic!("Test name can only be defined once.");
                        }
                        match &nv.lit {
                            syn::Lit::Str(_) => {
                                custom_name = Some(syn_helper::lit_to_str(&nv.lit));
                            }
                            _ => unimplemented!("Unexpected type for test name. Expected string."),
                        }
                    } else {
                        panic!("Unexpected identifier '{}'", identifier)
                    }
                }
            },
            syn::NestedMeta::Lit(lit) => {
                literals.push((*lit).clone());
            }
        }
    }
    TestCaseAttributes {
        literals,
        custom_name,
    }
}

fn calculate_test_name(attr: &TestCaseAttributes, fn_name: &str) -> String {
    let mut name = "".to_string();
    match &attr.custom_name {
        None => {
            name.push_str(fn_name);
            for lit in &attr.literals {
                name.push_str(&format!("_{}", syn_helper::lit_to_str(&lit)));
            }
        }
        Some(custom_name) => name = custom_name.to_string(),
    }
    name
}

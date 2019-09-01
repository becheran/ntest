//! Part of the ntest library. Add test cases to the rust test framework.

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::export::TokenStream2;
use syn::parse_macro_input;
mod syn_helper;

/// Test cases can be used to have multiple inputs for a given function.
/// With the *test_case* attribute multiple tests will be generated using the
/// [Procedural Macros](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html)
/// capabilities of rust.
///
/// The function input can be of type `int`, `bool`, or `str`.
///
/// Please note that test functions can only contain alphanumeric characters and '_' signs.
/// Special characters will be escaped using the '_' sign.
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
/// ```
///
/// Example with test_name attribute:
/// ```ignore
/// #[test_case(42, test_name="my_fancy_test")]
/// fn with_name(x: u32) {
///     assert_eq!(x, 42)
/// }
///```
#[proc_macro_attribute]
pub fn test_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let attribute_args = parse_macro_input!(attr as syn::AttributeArgs);

    let test_descriptions: Vec<TestDescription> =
        collect_test_descriptions(&input, &attribute_args);
    let fn_body = &input.block;
    let fn_args_idents = collect_function_arg_idents(&input);

    let mut result = TokenStream2::new();
    for test_description in test_descriptions {
        let test_case_name = syn::Ident::new(&test_description.name, Span::call_site());
        let literals = test_description.literals;
        if &literals.len() != &fn_args_idents.len() {
            panic!("Test case arguments and function input signature mismatch.");
        }

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

struct TestDescription {
    literals: Vec<syn::Lit>,
    name: String,
    // TODO add Meta attributes expected_result
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

fn collect_test_descriptions(
    input: &syn::ItemFn,
    attribute_args: &syn::AttributeArgs,
) -> Vec<TestDescription> {
    let mut test_case_descriptions: Vec<TestDescription> = vec![];
    let fn_name = input.sig.ident.to_string();
    test_case_descriptions.push(parse_test_case_attributes(&attribute_args, &fn_name));
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
                    test_case_descriptions
                        .push(parse_test_case_attributes(&argument_args, &fn_name));
                }
                syn::Meta::NameValue(_) => {
                    unimplemented!("Named values currently not supported.");
                }
            },
            Err(e) => panic!("Could not determine meta data. Error {}.", e),
        }
    }
    test_case_descriptions
}

fn parse_test_case_attributes(attr: &syn::AttributeArgs, fn_name: &str) -> TestDescription {
    let mut literals: Vec<syn::Lit> = vec![];
    let mut name = "".to_string();

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
                    if identifier == "test_name" {
                        if !name.is_empty() {
                            panic!("Test name can only be defined once.");
                        }
                        match &nv.lit {
                            syn::Lit::Str(_) => {
                                name = syn_helper::lit_to_str(&nv.lit);
                            }
                            _ => unimplemented!("Unexpected type for test_name. Expected string."),
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

    if name.is_empty() {
        name.push_str(fn_name);
        for lit in &literals {
            name.push_str(&format!("_{}", syn_helper::lit_to_str(&lit)));
        }
    }

    TestDescription { literals, name }
}

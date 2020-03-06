//! Part of the ntest library. Add timeout attribute to the rust test framework.

extern crate proc_macro;
extern crate syn;
use proc_macro2::{Ident, Span};

use proc_macro::TokenStream;
use quote::quote;

use syn::parse_macro_input;

/// The timeout attribute can be used for tests to let them fail if they exceed a certain execution time.
/// With the `#[timeout]` attribute a timeout in milliseconds is added to a test.
///
/// The function input must be of type `int`. For example `#[timeout(10)]` will fail if the test takes longer than 10 milliseconds.
///
/// # Examples
///
/// This example will not panic
///
/// ```
/// #[test]
/// #[timeout(100)]
/// fn no_timeout() {
///     let fifty_millis = time::Duration::from_millis(50);
///     thread::sleep(fifty_millis);
/// }
/// ```
///
/// This example will panic.
///
/// ```
/// #[test]
/// #[timeout(10)]
/// #[should_panic]
/// fn timeout() {
///     let fifty_millis = time::Duration::from_millis(50);
///     thread::sleep(fifty_millis);
/// }
/// ```
///
/// Also works with test functions using a Result:
///
/// ```
/// #[test]
/// #[timeout(100)]
/// fn timeout_with_result() -> Result<(), String> {
///     let ten_millis = time::Duration::from_millis(10);
///     thread::sleep(ten_millis);
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn timeout(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let attribute_args = parse_macro_input!(attr as syn::AttributeArgs);
    let sig = &input.sig;
    let sig_helper_ident = &Ident::new(
        &format!("{}_ntest_helper", input.sig.ident.to_string()),
        Span::call_site(),
    );
    let mut sig_helper = input.sig.clone();
    sig_helper.ident = (*sig_helper_ident).clone();
    let body = &input.block;
    let time_ms = get_timeout(&attribute_args);
    check_other_attributes(&input);
    let result = quote! {
        #sig {
            let ntest_timeout_now = std::time::Instant::now();
            let ntest_result = #sig_helper_ident();
            if ntest_timeout_now.elapsed().as_millis() > #time_ms {
                panic!("timeout: the function call took {} ms instead of {} ms", ntest_timeout_now.elapsed().as_millis(), #time_ms);
            }
            ntest_result
         }

        #sig_helper
        #body
    };
    result.into()
}

fn check_other_attributes(input: &syn::ItemFn) {
    for attribute in &input.attrs {
        let meta = attribute.parse_meta();
        match meta {
            Ok(m) => match m {
                syn::Meta::Path(p) => {
                    let identifier = p.get_ident().expect("Expected identifier!");
                    if identifier == "timeout" {
                        panic!("Timeout attribute is only allowed once");
                    }
                }
                syn::Meta::List(ml) => {
                    let identifier = ml.path.get_ident().expect("Expected identifier!");
                    if identifier == "timeout" {
                        panic!("Timeout attribute is only allowed once");
                    }
                }
                syn::Meta::NameValue(nv) => {
                    let identifier = nv.path.get_ident().expect("Expected identifier!");
                    if identifier == "timeout" {
                        panic!("Timeout attribute is only allowed once");
                    }
                }
            },
            Err(e) => panic!("Could not determine meta data. Error {}.", e),
        }
    }
}

fn get_timeout(attribute_args: &syn::AttributeArgs) -> u128 {
    if attribute_args.len() > 1 {
        panic!("Only one integer expected. Example: #[timeout(10)]");
    }
    match &attribute_args[0] {
        syn::NestedMeta::Meta(_) => {
            panic!("Integer expected. Example: #[timeout(10)]");
        }
        syn::NestedMeta::Lit(lit) => match lit {
            syn::Lit::Int(int) => return int.base10_parse::<u128>().expect("Integer expected"),
            _ => {
                panic!("Integer as timeout in ms expected. Example: #[timeout(10)]");
            }
        },
    }
}

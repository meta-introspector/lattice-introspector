use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn profile_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.sig.ident.to_string();
    let block = input_fn.block;
    let sig = input_fn.sig;
    let vis = input_fn.vis;
    let attrs = input_fn.attrs;

    let expanded = quote! {
        #(#attrs)* #vis #sig {
            scirs2_core::profiling::Timer::time_function(#fn_name, || {
                #block
            })
        }
    };

    TokenStream::from(expanded)
}
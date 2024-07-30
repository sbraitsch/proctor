extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;

    let expanded = quote! {
        #fn_sig {
            use std::time::{Instant, Duration};
            let __start = Instant::now();
            let __result = (|| {
                #fn_block
            })();
            println!("Result of {} => {:?} after {:?}.", stringify!(#fn_name), __result, __start.elapsed());
            __result
       }
    };

    expanded.into()
}

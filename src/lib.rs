extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let func_block = &input.block;
    let func_sig = &input.sig;

    let expanded = quote! {
        #func_sig {
            use std::time::{Instant, Duration};
            let start = Instant::now();
            let result = #func_block;
            let elapsed = start.elapsed();

            println!("Result of {} => {:?} after {:?}.", stringify!(#func_name), result, elapsed);
            result
       }
    };

    expanded.into()
}

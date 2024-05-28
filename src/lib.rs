#![doc = r#"
```rust
#[unsafe_fn_body::unsafe_fn_body]
fn unsafe_fn(){
    // ...
}
```

```rust
fn unsafe_fn(){
    unsafe{
        // ...
    }
}
```
"#]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn unsafe_fn_body(_: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item);

    let fn_name = input.sig.ident;
    let fn_args = input.sig.inputs;
    let fn_output = input.sig.output;
    let fn_body = input.block;
    let fn_vis = input.vis;
    let fn_abi = input.sig.abi;
    let fn_generics = input.sig.generics;
    let fn_asyncness = input.sig.asyncness;

    let expanded = quote! {
       #fn_vis #fn_asyncness #fn_abi fn #fn_name #fn_generics(#fn_args) #fn_output{
            unsafe #fn_body
       }
    };

    TokenStream::from(expanded)
}

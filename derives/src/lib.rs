mod generate_mixin;
mod generate_traitify;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn traitify(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_traitify::traitify(attr, item)
}

#[proc_macro_derive(Mixin)]
pub fn derive_mixin(input: TokenStream) -> TokenStream {
    generate_mixin::derive_mixin(input)
}

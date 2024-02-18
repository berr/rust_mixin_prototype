use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

pub fn derive_mixin(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Struct(s) = input.data else {
        panic!("Expected struct");
    };

    let mut required_impls = vec![];

    let type_name = &input.ident;

    for f in s.fields {
        let Type::Path(t) = &f.ty else {
            continue;
        };

        let Some(ident) = t.path.get_ident() else {
            continue;
        };

        if ident.to_string().ends_with("Mixin") {
            let field_name = f.ident.unwrap();
            required_impls.push((field_name, ident.clone()));
        }
    }

    required_impls
        .into_iter()
        .map(|(n, t)| generate_derive(n, t, type_name))
        .collect()
}

fn generate_derive(field_name: Ident, field_type: Ident, type_name: &Ident) -> TokenStream {
    quote! {
        impl MixinDelegate<#field_type> for #type_name {
            fn as_inner(&self) -> &#field_type { &self.#field_name }
            fn as_inner_mut(&mut self) -> &mut #field_type { &mut self.#field_name }
        }
    }
    .into()
}

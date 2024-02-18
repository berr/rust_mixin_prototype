use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, Block, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, Pat, TraitItemFn, Type,
    Visibility,
};

pub fn traitify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);

    let Type::Path(impl_type) = impl_block.self_ty.as_ref() else {
        panic!("Wrong type");
    };

    let original_name = impl_type.path.require_ident().unwrap();
    let ident_str = original_name.to_string();

    let expected_suffix = "Mixin";
    let new_name = if ident_str.ends_with(expected_suffix) {
        let (stem, _) = ident_str.split_at(ident_str.len() - expected_suffix.len());
        Ident::new(stem, Span::call_site().into())
    } else {
        panic!("Type must end in {expected_suffix}");
    };

    let trait_signatures: Vec<_> = impl_block
        .items
        .iter()
        .map(impl_item_to_signature)
        .collect();
    let trait_impl: Vec<_> = impl_block
        .items
        .iter()
        .map(impl_item_to_trait_fn_impl)
        .collect();
    let delegates_impl: Vec<_> = impl_block.items.iter().map(impl_delegate).collect();

    quote! {
        pub trait #new_name {
            #(#trait_signatures)*
        }

        impl #new_name for #original_name {
            #(#trait_impl)*
        }

        impl<T: MixinDelegate<#original_name>> #new_name for T {
            #(#delegates_impl)*
        }

    }
    .into()
}

fn impl_item_to_signature(item: &ImplItem) -> TraitItemFn {
    let ImplItem::Fn(function_definition) = item else {
        panic!("Accepts only function definitions");
    };

    TraitItemFn {
        attrs: vec![],
        sig: function_definition.sig.clone(),
        default: None,
        semi_token: None,
    }
}

fn impl_item_to_trait_fn_impl(item: &ImplItem) -> ImplItemFn {
    let ImplItem::Fn(function_definition) = item else {
        panic!("Accepts only function definitions");
    };

    ImplItemFn {
        vis: Visibility::Inherited,
        ..function_definition.clone()
    }
}

fn impl_delegate(item: &ImplItem) -> ImplItemFn {
    let ImplItem::Fn(function_definition) = item else {
        panic!("Accepts only function definitions");
    };

    let mut is_mut = false;

    let args: Vec<_> = function_definition
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(a) => {
                if let Pat::Ident(i) = a.pat.as_ref() {
                    Some(i.ident.clone())
                } else {
                    panic!("Expected identifiers");
                }
            }
            FnArg::Receiver(r) => {
                is_mut = r.mutability.is_some();
                None
            }
        })
        .collect();

    let method_name = &function_definition.sig.ident;

    let delegation_expression: proc_macro::TokenStream = if is_mut {
        quote! { { self.as_inner_mut().#method_name(#(#args),*) } }.into()
    } else {
        quote! { { self.as_inner().#method_name(#(#args),*) } }.into()
    };

    let b = syn::parse::<Block>(delegation_expression).unwrap();

    ImplItemFn {
        attrs: vec![],
        vis: Visibility::Inherited,
        defaultness: None,
        sig: function_definition.sig.clone(),
        block: b,
    }
}

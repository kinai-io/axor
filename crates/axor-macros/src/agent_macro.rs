use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemStruct};

use syn::{ItemImpl, Type, TypePath};


pub fn mark_agent_struct(input: TokenStream) -> TokenStream {
    let mut s = parse_macro_input!(input as ItemStruct);

    let has_default = has_default_derive(&s.attrs);

    if !has_default {
        s.attrs.push(syn::parse_quote!(#[derive(Default)]));
    }

    quote!(#s).into()
}

pub fn expand_agent_impl(input: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(input as ItemImpl);

    let self_ty = &item_impl.self_ty;
    let struct_ident = if let Type::Path(TypePath { path, .. }) = &**self_ty {
        path.get_ident().unwrap().clone()
    } else {
        panic!("#[agent_impl] must be used on an impl of a struct");
    };

    let gen = quote! {
        #item_impl

        impl crate::Agent for #struct_ident {
            fn name(&self) -> &'static str {
                stringify!(#struct_ident)
            }

            fn operations(&self) -> Vec<crate::OperationDescriptor> {
                vec![] // to be filled when #[operation] is implemented
            }

            fn inject_dependencies(&self, context: &crate::AxorContext) {
                // let default = Self::default();
                // default.inject_dependencies(context);
            }

            fn call_operation(&self, _payload: &crate::Payload) -> Option<serde_json::Value> {
                None // will be handled later
            }
        }
    };

    gen.into()
}

fn has_default_derive(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("derive") {
            let Ok(meta_list) = attr.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated) else {
                continue;
            };

            if meta_list.iter().any(|path| path.is_ident("Default")) {
                return true;
            }
        }
    }
    false
}
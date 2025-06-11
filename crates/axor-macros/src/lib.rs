use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn operation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;
    let fn_name_str = fn_name.to_string();
    let register_fn_name = format_ident!("__register_op_{}", fn_name);

    let gen = quote! {
        #vis #sig #block

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub fn #register_fn_name(agent_type_id: std::any::TypeId, map: &mut std::collections::HashMap<String, crate::OperationFn>) {
            let op_name = #fn_name_str.to_string();
            let op_fn = |agent: &dyn std::any::Any, _args: crate::Args| -> crate::OpResult {
                let agent = agent.downcast_ref::<Self>().ok_or("Invalid agent type")?;
                let result = agent.#fn_name();
                Ok(serde_json::to_value(result)?)
            };
            map.insert(op_name, Box::new(op_fn));
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn agent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let vis = &input.vis;
    let (impl_gen, ty_gen, where_clause) = input.generics.split_for_impl();

    let gen = quote! {
        #vis struct #struct_name #impl_gen #where_clause;

        impl #impl_gen crate::Agent for #struct_name #ty_gen #where_clause {
            fn operations(agent_type_id: std::any::TypeId, map: &mut std::collections::HashMap<String, crate::OperationFn>) {
                
                {
                    use crate::*;
                    let _ = agent_type_id;
                    
                }
            }
        }
    };

    gen.into()
}

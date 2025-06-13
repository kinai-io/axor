extern crate proc_macro;

mod agent_macro;
mod operation_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn agent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    agent_macro::mark_agent_struct(item)
}

#[proc_macro_attribute]
pub fn agent_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    agent_macro::expand_agent_impl(item)
}
#[proc_macro_attribute]
pub fn operation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    operation_macro::mark_operation(item)
}

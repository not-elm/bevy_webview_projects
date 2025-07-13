mod action_command;
mod async_command;

use crate::command::action_command::expand_action_command;
use crate::command::async_command::expand_async_command;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemFn;

pub fn expand_call_fn(f: &ItemFn) -> TokenStream2 {
    if f.sig.asyncness.is_some() {
        expand_async_command(f)
    } else {
        expand_action_command(f)
    }
}

enum Input {
    In,
    WebviewEntity,
    Task,
}

impl Input {
    pub fn to_token(&self) -> TokenStream2 {
        match self {
            Self::In => quote! {
                ipc_cmd.payload.deserialize_args()
            },
            Self::WebviewEntity => quote! {
                WebviewEntity(ipc_cmd.entity)
            },
            Self::Task => quote! {
                task.clone()
            },
        }
    }
}

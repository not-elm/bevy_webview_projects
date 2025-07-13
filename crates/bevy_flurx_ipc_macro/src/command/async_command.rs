use crate::command::Input;
use proc_macro2::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{FnArg, ItemFn, Type};

pub fn expand_async_command(f: &ItemFn) -> TokenStream2 {
    let fn_ident = &f.sig.ident;
    let inputs = parse_async_command_inputs(f);
    expand_call(quote! { #fn_ident(#(#inputs,)*).await; })
}

fn expand_call(f: TokenStream2) -> TokenStream2 {
    quote! {
        commands.spawn(bevy_flurx::prelude::Reactor::schedule(move |task| async move{
            let output = #f
            task.will(bevy::prelude::Update, bevy_flurx::prelude::once::event::send().with(IpcResolveEvent{
                    resolve_id: ipc_cmd.payload.resolve_id,
                    entity: ipc_cmd.entity,
                    output: to_string(output),
                })).await;
        }));
    }
}

fn parse_async_command_inputs(f: &ItemFn) -> Vec<TokenStream> {
    let mut args = Vec::with_capacity(3);
    for arg in f.sig.inputs.iter() {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };
        let Type::Path(path) = &*pat_type.ty else {
            continue;
        };
        let Some(last_segment) = path.path.segments.last() else {
            continue;
        };
        match last_segment.ident.to_string().as_str() {
            "In" => args.push(Input::In.to_token()),
            "WebviewEntity" => args.push(Input::WebviewEntity.to_token()),
            "ReactorTask" => args.push(Input::Task.to_token()),
            _ => continue,
        }
    }
    args
}

#![cfg_attr(docsrs, feature(doc_cfg))]

//! This crate provides macros to support `bevy_flurx_ipc`.

mod command;

use crate::command::expand_call_fn;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemFn;

/// Convert the function to `bevy_flurx_ipc-command`.
///
/// ## Parameters
///
/// - `id` to specify bevy_flurx_ipc-id: if not specified, bevy_flurx_ipc-id will be the same as the function name.  
///
/// ## Command Patterns
///
/// ### Action Command
///
/// The function that returns [`Action`] or [`ActionSeed`] is tentatively called `action command`.
///
/// The function has the following two arguments; each argument is optional.
/// -  [In](bevy::prelude::In)<D: [`DeserializeOwned`](serde::de::DeserializeOwned)>: The Deserialized values passed from the webview.
/// - `WebviewEntity` :  The webview entity that holds bevy_flurx_ipc-handlers.
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_flurx::prelude::*;
/// use bevy_flurx_ipc::prelude::*;
///
/// #[command]
/// fn action_command(In(args): In<String>, entity: WebviewEntity) -> Action<(String, WebviewEntity), String>{
///     once::run(|In(_): In<(String, WebviewEntity)>| "output is returned to Javascript".to_string()).with((args, entity))
/// }
/// ```
///
/// ### Async Command
///
/// Asynchronous functions that return output to Javascript are called `async command`.
///
/// The function has the following two arguments; each argument is optional.
/// -  [In](bevy::prelude::In)<D: [`DeserializeOwned`](serde::de::DeserializeOwned)>: The Deserialized values passed from the webview.
/// - `WebviewEntity` :  The webview entity that holds bevy_flurx_ipc-handlers.
/// - [`ReactorTask`]: Please see [here](https://docs.rs/bevy_flurx/latest/bevy_flurx/prelude/struct.Reactor.html#method.schedule) for details.
///  
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_flurx::prelude::*;
/// use bevy_flurx_ipc::prelude::*;
///
/// #[command]
/// async fn async_command(
///     In(message): In<String>,
///     _entity: WebviewEntity,
///     task: ReactorTask,
/// ) -> String{
///     task.will(Update, once::run(|In(message): In<String>|message).with(message)).await
/// }
/// ```
///
/// [`Action`]: https://docs.rs/bevy_flurx/latest/bevy_flurx/action/index.html
/// [`ActionSeed`]: https://docs.rs/bevy_flurx/latest/bevy_flurx/action/seed/struct.ActionSeed.html
/// [`ReactorTask`]: https://docs.rs/bevy_flurx/latest/bevy_flurx/task/struct.ReactorTask.html
#[proc_macro_attribute]
pub fn command(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attribute = parse_attribute(attr);
    parse_command(input, attribute)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn parse_command(input: TokenStream, attribute: Option<Attribute>) -> syn::Result<TokenStream2> {
    let custom_id = attribute.as_ref().and_then(|attr| attr.id.clone());
    let f = syn::parse::<ItemFn>(input)?;
    let fn_ident = &f.sig.ident.clone();
    let ipc_id = custom_id.unwrap_or(fn_ident.to_string());
    let call_fn = expand_call_fn(&f);
    let fn_ident = &f.sig.ident;
    let visibility = &f.vis;

    Ok(quote! {
        #[allow(missing_docs)]
        #visibility fn #fn_ident() -> IpcHandler{
            IpcHandler::new(#ipc_id, |commands, ipc_cmd|{
                #f
                #call_fn
            })
        }
    })
}

fn parse_attribute(attr: TokenStream) -> Option<Attribute> {
    let attr_args = NestedMeta::parse_meta_list(attr.into()).ok()?;
    Attribute::from_list(&attr_args).ok()
}

#[derive(Default, FromMeta)]
struct Attribute {
    id: Option<String>,
}

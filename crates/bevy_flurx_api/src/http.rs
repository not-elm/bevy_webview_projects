//! Allows you to use http apis.

use crate::error::http::DenyOrigin;
use crate::error::ApiResult;
use crate::macros::api_plugin;
use bevy::app::PluginGroupBuilder;
use bevy::platform::collections::hash_map::HashMap;
use bevy::prelude::{In, PluginGroup, Reflect, ReflectDefault, ReflectDeserialize, ReflectResource, ReflectSerialize, Res, Resource, Update};
use bevy_flurx::prelude::{once, side_effect, Pipe};
use bevy_flurx::task::ReactorTask;
use bevy_flurx_ipc::prelude::*;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Allows you to use all http plugins.
///
/// ## Plugins
/// - [HttpFetchApi]
pub struct AllHttpPlugins;
impl PluginGroup for AllHttpPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(HttpFetchApi)
    }
}

api_plugin!(
    /// You'll be able to use fetch bevy_flurx_api.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const response = await window.__FLURX__.http.fetch("https://example.com");
    /// ```
    HttpFetchApi,
    command: fetch
);

#[derive(Deserialize, Debug, Default)]
struct Args {
    url: String,
    body: Option<Vec<u8>>,
    method: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl Args {
    async fn fetch(self) -> ApiResult<Response> {
        let mut client = reqwest::Client::new().request(self.method()?, &self.url);
        if let Some(headers) = self.headers {
            client = client.headers(to_header_map(&headers)?);
        }
        if let Some(body) = self.body {
            client = client.body(body);
        }
        Ok(client.send().await?)
    }

    fn method(&self) -> ApiResult<Method> {
        match self.method.as_ref() {
            Some(method) => Ok(Method::from_str(method.as_str())?),
            None => Ok(Method::GET),
        }
    }
}

#[derive(Serialize)]
struct Output {
    body: Vec<u8>,
    headers: HashMap<String, String>,
    status: u16,
    #[serde(rename = "statusText")]
    status_text: String,
}

/// Defines the list of origins allowed to access urls from http apis.
///
/// If this resource is existing, it allows　access only allowed origins.
/// If it does not exist, access is allowed from all origins.
#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Resource, Default, Serialize, Deserialize)]
pub struct AccessAllowOrigins(Vec<String>);
impl AccessAllowOrigins {
    /// Creates the new [`AccessAllowOrigins`].
    ///
    /// ## Examples
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_webview_wry::prelude::*;
    /// use bevy_flurx_api::prelude::AccessAllowOrigins;
    ///
    /// let mut app = App::new();
    /// app.insert_resource(AccessAllowOrigins::new([
    ///     "https://example.com/"
    /// ]));
    /// ```
    pub fn new<O>(origins: impl IntoIterator<Item=O>) -> Self
    where
        O: Into<String>,
    {
        AccessAllowOrigins(origins.into_iter().map(O::into).collect())
    }

    /// Returns whether the url is permitted.
    pub fn is_allow(&self, url: &str) -> bool {
        self.0.iter().any(|origin| {
            url.starts_with(origin.as_str())
        })
    }
}

#[command(id = "FLURX|http::fetch")]
async fn fetch(In(args): In<Args>, task: ReactorTask) -> ApiResult<Output> {
    task.will(
        Update,
        once::run(error_if_deny_access).with(args)
            .pipe(side_effect::tokio::spawn(|args: ApiResult<Args>| async move {
                let args = args?;
                let response = args.fetch().await?;
                let status = response.status();
                Ok(Output {
                    headers: to_hash_map(response.headers()),
                    body: response.bytes().await?.to_vec(),
                    status: status.as_u16(),
                    status_text: status.to_string(),
                })
            })),
    )
        .await
}

fn error_if_deny_access(
    In(args): In<Args>,
    allow_origins: Option<Res<AccessAllowOrigins>>,
) -> ApiResult<Args> {
    match allow_origins {
        Some(allow_origins) => {
            if allow_origins.is_allow(args.url.as_str()) {
                Ok(args)
            } else {
                Err(DenyOrigin(args.url.clone()).into())
            }
        }
        None => Ok(args),
    }
}

fn to_header_map(headers: &HashMap<String, String>) -> ApiResult<HeaderMap> {
    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        header_map.insert(HeaderName::from_str(name)?, value.parse()?);
    }
    Ok(header_map)
}

fn to_hash_map(headers: &HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .filter_map(|(k, v)| Some((k.to_string(), v.to_str().ok()?.to_string())))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::error::ApiResult;
    use crate::http::{error_if_deny_access, AccessAllowOrigins, Args};
    use crate::tests::test_app;
    use bevy::app::{Startup, Update};
    use bevy::prelude::Commands;
    use bevy::utils::default;
    use bevy_flurx::prelude::*;
    use reqwest::Method;

    #[test]
    fn create_methods() -> ApiResult {
        assert_eq!(
            Args {
                method: None,
                ..default()
            }
                .method()?,
            Method::GET
        );
        assert_eq!(
            Args {
                method: Some("GET".to_string()),
                ..default()
            }
                .method()?,
            Method::GET
        );
        assert_eq!(
            Args {
                method: Some("POST".to_string()),
                ..default()
            }
                .method()?,
            Method::POST
        );
        assert_eq!(
            Args {
                method: Some("PUT".to_string()),
                ..default()
            }
                .method()?,
            Method::PUT
        );
        assert_eq!(
            Args {
                method: Some("DELETE".to_string()),
                ..default()
            }
                .method()?,
            Method::DELETE
        );
        Ok(())
    }

    #[test]
    fn deny_origins() {
        let origins = AccessAllowOrigins::new([
            "https://example.com",
        ]);
        assert!(!origins.is_allow("https://hoge.com"));
    }

    #[test]
    fn allow_origins() {
        let origins = AccessAllowOrigins::new([
            "https://example.com",
            "https://hoge.com/",
        ]);
        assert!(origins.is_allow("https://example.com"));
        assert!(origins.is_allow("https://hoge.com/index.html"));
    }

    #[test]
    fn output_ok_if_not_exists_allow_origins() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let result = task.will(Update, once::run(error_if_deny_access).with(Args {
                    url: "https://hoge.com".to_string(),
                    ..default()
                })).await;
                result.expect("Expected to return Result::ok but was Err.");
            }));
        });
        app.update();
    }

    #[test]
    fn output_ok_if_allow_origin() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let result = task.will(Update, {
                    once::res::insert().with(AccessAllowOrigins::new([
                        "https://example.com",
                    ]))
                        .then(once::run(error_if_deny_access).with(Args {
                            url: "https://example.com".to_string(),
                            ..default()
                        }))
                }).await;
                result.expect("Expected to return Result::ok but was error.");
            }));
        });
        app.update();
    }

    #[test]
    fn output_error_if_deny_origin() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let result: ApiResult<_> = task.will(Update, {
                    once::res::insert().with(AccessAllowOrigins::new([
                        "https://example.com",
                    ]))
                        .then(once::run(error_if_deny_access).with(Args {
                            url: "https://hoge.com".to_string(),
                            ..default()
                        }))
                }).await;
                result.expect_err("Expected to return Err but was Ok.");
            }));
        });
        app.update();
    }
}

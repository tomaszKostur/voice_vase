use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use leptos::config::LeptosOptions;
use leptos_router::Method::Post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub use error::{Error, Result};

//////////////////////////
#[derive(Debug, Deserialize)]
pub struct SandboxParams {
    name: Option<String>,
}

pub async fn hello_axum_sandbox(Query(params): Query<SandboxParams>) -> impl IntoResponse {
    println!(
        "->> {:<12} log from hello_axum_sandbox: params: {params:?}",
        "HANDLER"
    );
    let name = params.name.as_deref().unwrap_or("PLACEHOLDER");
    Html(format!("Axum sandbox hello <string>{name}</strong>"))
}

//////////////////////////////
/// api login
///

#[derive(Debug, Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}

async fn api_login(payload: Json<LoginParams>) -> Result<Json<Value>> {
    println!(
        "->> {:<12} log from hello_axum_sandbox: params: {payload:?}",
        "API LOGIN"
    );
    // credential check placeholder
    if payload.username != "tomasz" || payload.password != "impossible" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({"result":{"success": true}}));
    Ok(body)
}

//////////////////////////

pub fn get_axum_sandbox_router() -> Router<LeptosOptions> {
    Router::new()
        .route("/axum_sb", get(hello_axum_sandbox))
        .route("/axum_sb/login", post(api_login))
}

mod error {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error {
        LoginFail,
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            println!("->> {:<12} - {self:?}", "INTO_RES");
            (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
        }
    }
}

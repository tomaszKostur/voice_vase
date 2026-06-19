use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use leptos::config::LeptosOptions;
use serde::Deserialize;

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

pub fn get_axum_sandbox_router() -> Router<LeptosOptions> {
    Router::new().route("/axum_sb", get(hello_axum_sandbox))
}

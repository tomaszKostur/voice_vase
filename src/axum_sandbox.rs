use axum::response::IntoResponse;

pub async fn hello_axum_sandbox() -> impl IntoResponse {
    "Hello axum sandbox"
}
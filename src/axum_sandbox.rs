use axum::response::IntoResponse;

pub async fn hello_axum_sandbox() -> impl IntoResponse {
    println!("log from hello_axum_sandbox");
    "Hello axum sandbox"
}
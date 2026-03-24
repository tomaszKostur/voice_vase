use std::result;

use leptos::prelude::*;


#[cfg(feature = "ssr")]
pub mod ssr {
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db_connection() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:voice_base.db").await?)
    }
}


#[server]
pub async fn trigger_log_on_server() -> Result<(), ServerFnError> {
    println!("Sanity check log");
    Ok(())
}

#[server]
pub async fn get_example_data_from_db() -> Result<String, ServerFnError> {
    let mut db = ssr::db_connection().await.expect("Cannot connect to DB");
    let result = sqlx::query!("select email from actor_table;").fetch_one(&mut db).await?;
    Ok(String::from(format!("{result:?}")))
}
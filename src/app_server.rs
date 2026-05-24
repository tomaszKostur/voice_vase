

use std::time::Duration;
use serde::{Serialize, Deserialize};

use leptos::prelude::*;
// use tokio::time::sleep;


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
    leptos::logging::debug_log!("From server get_example_data_from_db");
    Ok(String::from(format!("{result:?}")))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    pub email: String,
    pub about: String,
    pub cooperation: String
}

#[server]
pub async fn get_list_of_all_actors() -> Result<Vec<Actor>, ServerFnError> {
    let act = Actor{email: String::from("Email"),
                     about: String::from("About"),
                     cooperation: String::from("Coop")};
    Ok(vec![act])
}

#[server]
pub async fn get_server_greeting(name: String) -> Result<String, ServerFnError> {
    Ok(format!("greetings {name}!"))
}

#[server]
pub async fn get_error_from_server() -> Result<String, ServerFnError>{
    Err(ServerFnError::new("Error forced from server"))
}
#![allow(unused)] // Not for production
#![cfg(feature = "ssr")]

use serde_json::json;
use anyhow::Result;

// WARNING: This are dev tests. The tev tests need server to be setup manually

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/axum_sb").await?.print().await?;
    hc.do_get("/axum_sb?name=Tomasz").await?.print().await?;
    Ok(())
}


#[tokio::test]
async fn login_cycle() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/axum_sb").await?.print().await?;
    hc.do_post("/axum_sb/login", json!({"username": "tomas", "password": "impossible"})).await?.print().await?;
    Ok(())
}
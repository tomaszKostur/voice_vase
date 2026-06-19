#![allow(unused)] // Not for production
#![cfg(feature = "ssr")]


use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/axum_sb").await?.print().await?;

    hc.do_get("/axum_sb?name=Tomasz").await?.print().await?;

    Ok(())
}
#[allow(unused)]
use anyhow::Result;
use serde_json::json;

//cargo watch -q  -c -w tests/ -x "test --features httpc-test"
//cargo test --package axum-api --test http -- quick_dev_test --exact --show-output
#[tokio::test]
async fn quick_dev_test() -> Result<()> {
    //region: -- start server --
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    //endregion: -- start server --

    hc.do_get("/hello?name=Tew").await?.print().await?;

    hc.do_get("/hello2/Tew").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc
        .do_post(
            "/api/login",
            json!({
                "username": "admin",
                "password": "admin"
            }),
        )
        .await?;

    req_login.print().await?;

    Ok(())
}

#[allow(unused)]
use anyhow::Result;

//cargo watch -q  -c -w tests/ -x "test --features httpc-test"
//cargo test --package axum-api --test http -- quick_dev_test --exact --show-output
#[tokio::test]
async fn quick_dev_test() -> Result<()> {
    //region: -- start server --
    let hc = httpc_test::new_client("http://localhost:8080/hello/?name=Tew")?;
    //endregion: -- start server --

    hc.do_get("/hello").await?.print().await?;
    Ok(())
}


use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/with_closure").await?.print().await?;
    client.do_get("/with_async_fun").await?.print().await?;
    client.do_get("/with_query_args?name=Bob").await?.print().await?;
    client.do_get("/with_path_args/Bob").await?.print().await?;
    
    Ok(())
}
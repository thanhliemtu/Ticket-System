#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {

    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Testing GET query params
    hc.do_get("/hello?name=Haruka").await?.print().await?;

    // Testing GET path params
    hc.do_get("/hello2/Hansamu").await?.print().await?;

    // Testing POST (uncomment the line below to test)
    // hc.do_post("/hello3", "Hello from POST request").await?.print().await?;

    // Testing static routing (uncomment the line below to test)
    // hc.do_get("/commands.txt").await?.print().await?;

    Ok(())
}
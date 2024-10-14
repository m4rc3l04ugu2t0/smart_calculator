use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    let calc = hc.do_post(
        "/calculate",
        json!({
            "expression": "2+2",
        }),
    );

    calc.await?.print().await?;

    Ok(())
}

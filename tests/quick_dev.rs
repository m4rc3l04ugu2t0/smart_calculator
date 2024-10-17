use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    // Lista de expressões para teste
    let expressions = vec![
        "2+2",
        "10-5",
        "3*3",
        "8/4",
        "(1+2)*(3+4)",
        "2^3",
        "(9+1)r2", // Raiz quadrada de 10
        "-5+10",
        "7-(-3)",
        "3r3", // Raiz cúbica de 3
    ];

    for expression in expressions {
        let calc = hc.do_post(
            "/calculate",
            json!({
                "expression": expression,
            }),
        );

        println!("Testing expression: {}", expression);
        calc.await?.print().await?;
    }

    Ok(())
}

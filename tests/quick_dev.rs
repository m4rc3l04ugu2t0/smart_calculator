use anyhow::{Error, Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<(), Error> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    // Lista de expressões para teste e seus resultados esperados
    let test_cases = vec![
        ("2+2", 4.0),
        ("10-5", 5.0),
        ("3*3", 9.0),
        ("8/4", 2.0),
        ("(1+2)*(3+4)", 21.0),
        ("2^3", 8.0),
        ("(9+1)r2", 3.1622776601683795), // Raiz quadrada de 10
        ("-5+10", 5.0),
        ("7-(-3)", 10.0),
        ("3r3", 1.4422495703074083), // Raiz cúbica de 3
        ("5+5*2", 15.0),
        ("(5+5)*2", 20.0),
        ("4/(2-1)", 4.0),
        ("6-4/2", 4.0),
        ("10*(2+3)", 50.0),
        ("(3+2)^2", 25.0),
        ("2^3*2", 16.0),
        ("(6-4)*(3+2)", 10.0),
        ("10-2*(3+4)", -4.0),
        ("3+4*2-1", 10.0),
        ("2*(3+4)-1", 13.0),
        ("1+2+3+4+5+6+7+8+9+10", 55.0),
        ("2*(3-4)", -2.0),
        ("(2+3)*5-10/2", 20.0),
        ("8/2+3*4", 16.0),
        ("10*(3+5)/4", 20.0),
        ("(10-2)*(5+5)", 80.0),
        ("2*5-3*(1+2)", 1.0),
        ("12/(4-2)", 6.0),
        ("2^4-4^2", 0.0),
        ("-10+5", -5.0),
        ("2*(3+1)", 8.0),
        ("100/10-5", 5.0),
        ("0+1", 1.0),
        ("1-0", 1.0),
        ("4*(1-0.5)", 2.0),
        ("(2+3)/(1+1)", 2.5),
        ("5-(-5)", 10.0),
        ("(2*3)+4", 10.0),
        ("6+6/2", 9.0),
        ("9-(3+4)", 2.0),
        ("10-5+1", 6.0),
        ("100/(10-5)", 20.0),
        ("2^3*3", 24.0),
        ("3*(4+2)", 18.0),
        ("0.5+0.5", 1.0),
        ("(1.5+1.5)*2", 6.0),
        ("7*2-3", 11.0),
        ("(1+2)*4-1", 11.0),
        ("2*5-3/2", 8.5),
        ("6/(2+1)", 2.0),
        ("0-5", -5.0),
        ("(3+3)^3", 216.0),
        ("10*(1-0.1)", 9.0),
        ("(10-5)^2", 25.0),
        ("5*2^2", 20.0),
        ("2^0", 1.0),
        ("1+(-1)+1", 1.0),
        ("(-2)*(3+4)", -14.0),
        ("3-(-3)", 6.0),
        ("(5-3)^3", 8.0),
        ("10/(1+1)", 5.0),
        ("2*(3+4)-5", 9.0),
        ("3^2-1", 8.0),
        ("1+0", 1.0),
        ("(5+5)/(1+1)", 5.0),
        ("10-5^2", -15.0),
        ("6/(2+1)", 2.0),
        ("0.5*(4+4)", 4.0),
        ("(8-4)*2", 8.0),
        ("1+1*1", 2.0),
        ("4-2*(1+1)", 0.0),
        ("(2-1)*3", 3.0),
        ("10/5+1", 3.0),
        ("3+4*2-7", 4.0),
        ("0.5+0.5", 1.0),
        ("(3+1)*(2-2)", 0.0),
        ("2*(3-1)", 4.0),
        ("10-(-3)", 13.0),
        ("(1+2)*(4+5)", 27.0),
        ("5-2*2", 1.0),
        ("4*(2+1)", 12.0),
        ("10-5-5", 0.0),
        ("-9r3", 0.0),
    ];

    for (expression, expected_result) in test_cases {
        let calc = hc.do_post(
            "/calculate",
            json!({
                "expression": expression,
            }),
        );

        let response = calc.await?;
        let response_json = response.json_body().expect("kks");

        // Obtém o resultado do JSON
        let result = response_json["result"].as_f64().unwrap_or(0.0);
        let status = response_json["status"].as_str().unwrap_or("Unknown");

        println!("{}", status);
        // Verifica se o resultado está correto
        if result == expected_result {
            println!(
                "Test passed for expression: {} with result: {}",
                expression, result
            );
        } else {
            let msg_err = format!(
                "Test failed for expression: {}. Expected: {}, Got: {}, Status: {}",
                expression, expected_result, result, status
            );
            println!("{}", msg_err);
            return Err(anyhow::anyhow!(
                "Test failed for expression: {}",
                expression
            ));
        }
    }

    Ok(())
}

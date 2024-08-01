mod check_expression;
mod error;
mod evaluator;
mod parse_expression;
mod structs;
mod web;

use axum::{routing::post, serve, Router};
use tokio::net::TcpListener;
use web::entry_point::calculate;

pub use self::error::{ClientError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/calculate", post(calculate));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();

    Ok(())
}

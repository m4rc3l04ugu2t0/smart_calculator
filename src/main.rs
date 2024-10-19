mod check_expression;
mod error;
mod evaluator;
mod parse_expression;
mod structs;
mod web;

use axum::{routing::post, Router};
use std::{env::var, net::SocketAddr};
use web::entry_point::calculate;

pub use self::error::{ClientError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let port = var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let app = Router::new().route("/calculate", post(calculate));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

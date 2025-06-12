#![allow(unused)]

mod config;
mod constants;
mod ctx;
mod db;
mod error;
mod macros;
mod routes;
mod services;

use axum::{Router, routing::get};
use error::Result;
use routes::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = routes().await?;

    //todo: add cors
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

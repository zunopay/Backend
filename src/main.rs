#![allow(unused)]

mod config;
mod constants;
mod ctx;
mod db;
mod error;
mod macros;
mod routes;
mod services;

use axum::{
    Router,
    http::{HeaderName, HeaderValue},
    routing::get,
};
use error::{Error, Result};
use reqwest::{
    Method,
    header::{
        ACCEPT_ENCODING, ACCESS_CONTROL_ALLOW_HEADERS, AUTHORIZATION, CONTENT_ENCODING,
        CONTENT_TYPE,
    },
};
use routes::*;
use std::time::Duration;
use tokio::signal::{self, unix::SignalKind};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, Trace, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    //todo: add X-API-KEY header when start using it
    //CORS layer
    let cors =
        CorsLayer::new()
            .allow_origin("*".parse::<HeaderValue>().map_err(|_| {
                Error::MiddlewareError("Failed to parse allowed origin haeder value")
            })?)
            .allow_headers([
                AUTHORIZATION,
                CONTENT_TYPE,
                ACCEPT_ENCODING, // To get supported response compression encoding
                ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderName::from_static("x-requested-with"),
            ])
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::DELETE,
                Method::OPTIONS,
            ]);

    // Trace layer
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let app = routes().await?;
    let app = app
        //Response Compression layer (Brotli)
        .layer(CompressionLayer::new())
        //Request trace logs
        .layer(trace_layer)
        .layer(cors)
        //Request timeout after 30s
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CatchPanicLayer::new());

    let port = 8000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map_err(|e| Error::FailedToBindListener {
            port,
            e: e.to_string(),
        })?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| Error::FailedToBindListener {
            port,
            e: e.to_string(),
        })?;

    Ok(())
}

//todo: add cleanups
async fn shutdown_signal() {
    let cntrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = cntrl_c => {
            println!("Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            println!("Received SIGTERM, shutting down...");
        }
    }

    println!("Shutdown signal received");
}

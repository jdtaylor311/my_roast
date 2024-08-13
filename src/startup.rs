//! src/startup.rs

use axum::{serve::Serve, Router};
use tokio::net::TcpListener;

#[derive()]
struct Application {
    app: Serve<TcpListener, Router>,
    port: u16,
}

use axum::{routing::get, Router};
use std::future::pending;
use std::net::SocketAddr;

pub async fn run_server() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    println!("Starting server at http://localhost:3030/...");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(pending::<()>())
        .await
        .expect("server error");
}

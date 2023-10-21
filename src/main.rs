use std::{sync::Arc, net::SocketAddr};

use database::database_connection;
use router::global_router;
use tokio::sync::Mutex;


mod router;
mod database;
mod model;
mod service;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(database_connection().await));
    let app = global_router(db);

    let socket = SocketAddr::from(([0,0,0,0], 8080));
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


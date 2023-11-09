use std::{net::SocketAddr, sync::Arc};

use database::database_connection;
use router::global_router;
use tokio::sync::Mutex;

mod database;
mod layer;
mod model;
mod router;
mod service;

#[tokio::main]
async fn main() {
    let db = Arc::new(database_connection());
    let app = global_router(db);

    let socket = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

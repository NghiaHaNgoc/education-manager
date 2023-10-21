use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::service::public_service::login;

fn public_router(database: Arc<Mutex<Surreal<Client>>>) -> Router {
Router::new().route("/login", post(login)).with_state(database)
}

pub fn global_router(database: Arc<Mutex<Surreal<Client>>>) -> Router {
Router::new().nest("/fer201m/api", public_router(database.clone())).layer(CorsLayer::very_permissive())
}

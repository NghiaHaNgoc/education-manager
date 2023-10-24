use std::sync::Arc;

use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use surrealdb::engine::remote::http::Client;
use surrealdb::Surreal;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::layer::{admin_layer, extract_authorization};
use crate::service::general_service::profile;
use crate::service::public_service::login;

fn public_router(database: Arc<Mutex<Surreal<Client>>>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(database)
}

pub fn global_router(database: Arc<Mutex<Surreal<Client>>>) -> Router {
    let router = Router::new()
        .merge(public_router(database.clone()))
        .merge(authentication_router(database.clone()));

    Router::new()
        .nest("/fer201m/api", router)
        .layer(CorsLayer::very_permissive())
}

fn authentication_router(database: Arc<Mutex<Surreal<Client>>>) -> Router {
    Router::new()
        .route("/profile", get(profile))
        .with_state(database.clone())
        .merge(admin_router(database.clone()))
        .route_layer(middleware::from_fn_with_state(
            database.clone(),
            extract_authorization,
        ))
}

fn admin_router(database: Arc<Mutex<Surreal<Client>>>) -> Router {
    Router::new()
        .route("/list-students", get(foo))
        .with_state(database.clone())
        .route_layer(middleware::from_fn(admin_layer))
}
async fn foo() -> String {
    "helllo".to_string()
}

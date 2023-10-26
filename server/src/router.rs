use std::sync::Arc;

use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use postgrest::Postgrest;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::layer::{admin_layer, extract_authorization};
use crate::service::admin_service::list_student::list_student;
use crate::service::general_service::profile::profile;
use crate::service::public_service::login;

fn public_router(database: Arc<Mutex<Postgrest>>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(database)
}

pub fn global_router(database: Arc<Mutex<Postgrest>>) -> Router {
    let router = Router::new()
        .merge(public_router(database.clone()))
        .merge(authentication_router(database.clone()));

    Router::new()
        .nest("/fer201m/api", router)
        .layer(CorsLayer::very_permissive())
}

fn authentication_router(database: Arc<Mutex<Postgrest>>) -> Router {
    Router::new()
        .route("/profile", get(profile))
        .with_state(database.clone())
        .merge(admin_router(database.clone()))
        .route_layer(middleware::from_fn_with_state(
            database.clone(),
            extract_authorization,
        ))
}

fn admin_router(database: Arc<Mutex<Postgrest>>) -> Router {
    Router::new().nest(
        "/admin",
        Router::new()
            .route("/students-list", get(list_student))
            .with_state(database.clone())
            .route_layer(middleware::from_fn(admin_layer)),
    )
}

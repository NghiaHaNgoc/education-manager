use std::sync::Arc;

use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;

use crate::layer::{admin_layer, extract_authorization};
use crate::service::admin_service::list_lecturer::list_lecturer;
use crate::service::admin_service::list_student::list_student;
use crate::service::admin_service::{
    class_detail, create_class, create_user, lecturer_detail, list_class, remove_user,
    student_detail, update_class,
};
use crate::service::general_service::profile::profile;
use crate::service::general_service::update_profile;
use crate::service::public_service::login;

pub fn global_router(database: Arc<Postgrest>) -> Router {
    let router = Router::new()
        .merge(public_router(database.clone()))
        .merge(authentication_router(database.clone()));

    Router::new()
        .nest("/fer201m/api", router)
        .layer(CorsLayer::very_permissive())
}

fn public_router(database: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(database)
}

fn authentication_router(database: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/profile", get(profile))
        .route("/update-profile", post(update_profile::update_profile))
        .with_state(database.clone())
        .merge(admin_router(database.clone()))
        .route_layer(middleware::from_fn_with_state(
            database.clone(),
            extract_authorization,
        ))
}

fn admin_router(database: Arc<Postgrest>) -> Router {
    Router::new().nest(
        "/admin",
        Router::new()
            .route("/students-list", get(list_student))
            .route("/lecturers-list", get(list_lecturer))
            .route("/classes-list", get(list_class::list_class))
            .route(
                "/student-detail/:student_id",
                get(student_detail::student_detail),
            )
            .route(
                "/lecturer-detail/:lecturer_id",
                get(lecturer_detail::lecturer_detail),
            )
            .route(
                "/class-detail/:current_class_code",
                get(class_detail::class_detail),
            )
            .route("/create-user", post(create_user::create_user))
            .route("/create-class", post(create_class::create_class))
            .route(
                "/update-class/:current_class_code",
                post(update_class::update_class),
            )
            .route("/remove-user", post(remove_user::remove_user))
            .with_state(database.clone())
            .route_layer(middleware::from_fn(admin_layer)),
    )
}

use std::sync::Arc;

use axum::http::{header, HeaderValue};
use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;

use crate::layer::{self, admin_layer, extract_authorization};
use crate::service::admin_service::list_lecturer::list_lecturer;
use crate::service::admin_service::list_student::list_student;
use crate::service::admin_service::{
    add_lecturers_to_class, add_students_to_class, class_detail, create_class, create_user,
    lecturer_detail, list_class, remove_lecturers_from_class, remove_students_from_class,
    remove_user, student_detail, update_class, student_not_in_any_class, lecturer_not_in_any_class, remove_class,
};
use crate::service::general_service::profile::profile;
use crate::service::general_service::update_profile;
use crate::service::public_service::login;
use crate::service::{student_service, lecturer_service};

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
        .merge(lecturer_router(database.clone()))
        .merge(student_router(database.clone()))
        .route_layer(middleware::from_fn_with_state(
            database,
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
            .route("/student-not-in-any-class", get(student_not_in_any_class::student_not_in_any_class))
            .route("/lecturer-not-in-any-class", get(lecturer_not_in_any_class::lecturer_not_in_any_class))
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
                "/add-students-to-class",
                post(add_students_to_class::add_students_to_class),
            )
            .route(
                "/add-lecturers-to-class",
                post(add_lecturers_to_class::add_lecturers_to_class),
            )
            .route(
                "/update-class/:current_class_code",
                post(update_class::update_class),
            )
            .route("/remove-user", post(remove_user::remove_user))
            .route("/remove-class", post(remove_class::remove_class))
            .route(
                "/remove-students-from-class",
                post(remove_students_from_class::remove_students_from_class),
            )
            .route(
                "/remove-lecturers-from-class",
                post(remove_lecturers_from_class::remove_lecturers_from_class),
            )
            .with_state(database)
            .route_layer(middleware::from_fn(admin_layer)),
    )
}

fn lecturer_router(database: Arc<Postgrest>) -> Router {
    Router::new().nest(
        "/lecturer",
        Router::new()
        .route("/class-detail/:current_class_code", get(lecturer_service::class_detail::class_detail))
            .with_state(database)
            .route_layer(middleware::from_fn(layer::lecturer_layer)),
    )
}

fn student_router(database: Arc<Postgrest>) -> Router {
    Router::new().nest(
        "/student",
        Router::new()
        .route("/class-detail/:current_class_code", get(student_service::class_detail::class_detail))
            .with_state(database)
            .route_layer(middleware::from_fn(layer::student_layer)),
    )
}

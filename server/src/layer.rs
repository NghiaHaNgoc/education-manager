use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response}, Extension,
};
use jsonwebtoken::{DecodingKey, Validation};
use surrealdb::{engine::remote::http::Client, opt::RecordId, Surreal};
use tokio::sync::Mutex;

use crate::model::{
    database_model::{Account, Role}, ErrorMessage, GeneralResponse, TokenClaims, SECRECT_KEY,
};

pub async fn extract_authorization<B>(
    State(db): State<Arc<Mutex<Surreal<Client>>>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    let token = match get_header_auth(&req) {
        Some(tk) => tk,
        None => {
            return GeneralResponse::new(
                StatusCode::UNAUTHORIZED,
                None,
                ErrorMessage::new(StatusCode::UNAUTHORIZED, String::from("Unautorrized!"))
                    .to_json(),
            )
            .into_response()
        }
    };
    let claims = match jsonwebtoken::decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(SECRECT_KEY.as_bytes()),
        &Validation::default(),
    ) {
        Ok(claim_data) => claim_data.claims,
        Err(err) => {
            return GeneralResponse::new(
                StatusCode::UNAUTHORIZED,
                None,
                ErrorMessage::new(StatusCode::UNAUTHORIZED, err.to_string()).to_json(),
            )
            .into_response()
        }
    };

    let query = format!(
        "SELECT role, user_profile from accounts where role = \"{}\" && user_profile = \"{}\"",
        claims.role, claims.id
    );
    println!("{}", query);
    match db
        .lock()
        .await
        .query(query)
        .await
        .unwrap()
        .take::<Option<Account>>(0)
    {
        Ok(result) => match result {
            None => {
                return GeneralResponse::new(
                    StatusCode::UNAUTHORIZED,
                    None,
                    ErrorMessage::new(StatusCode::UNAUTHORIZED, String::from("Unautorrized!"))
                        .to_json(),
                )
                .into_response()
            }
            Some(_) => (),
        },
        Err(err) => {
            return GeneralResponse::new(
                StatusCode::UNAUTHORIZED,
                None,
                ErrorMessage::new(StatusCode::UNAUTHORIZED, err.to_string()).to_json(),
            )
            .into_response()
        }
    };
    req.extensions_mut().insert(claims);

    next.run(req).await.into_response()
    // GeneralResponse::new(StatusCode::UNAUTHORIZED, None, "{\"message\" : \"Ok\"}".to_string())
}

fn get_header_auth<B>(req: &Request<B>) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
}

pub async fn student_layer<B>(Extension(user_claims): Extension<TokenClaims>,req: Request<B>, next: Next<B>) -> Response {
    if user_claims.role == Role::STUDENT {
        next.run(req).await
    } else {
        GeneralResponse::unauthorized().into_response()
    }
}
pub async fn teacher_layer<B>(Extension(user_claims): Extension<TokenClaims>,req: Request<B>, next: Next<B>) -> Response {
    if user_claims.role == Role::TEACHER {
        next.run(req).await
    } else {
        GeneralResponse::unauthorized().into_response()
    }
}
pub async fn admin_layer<B>(Extension(user_claims): Extension<TokenClaims>,req: Request<B>, next: Next<B>) -> Response {
    if user_claims.role == Role::ADMIN {
        next.run(req).await
    } else {
        GeneralResponse::unauthorized().into_response()
    }
}

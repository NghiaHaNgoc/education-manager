use std::sync::Arc;

use axum::extract::State;
use axum::http::{header, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use jsonwebtoken::{DecodingKey, Validation};
use postgrest::Postgrest;

use crate::model::{
    database_model::{Admin, Lecturer, Role, Student},
    GeneralResponse, TokenClaims, SECRECT_KEY,
};

pub async fn extract_authorization<B>(
    State(db): State<Arc<Postgrest>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    let token = match get_header_auth(&req) {
        Some(tk) => tk,
        None => return GeneralResponse::unauthorized(None).into_response(),
    };
    let claims = match jsonwebtoken::decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(SECRECT_KEY.as_bytes()),
        &Validation::default(),
    ) {
        Ok(claim_data) => claim_data.claims,
        Err(err) => return GeneralResponse::unauthorized(Some(err.to_string())).into_response(),
    };

    match claims.role {
        Role::Student => {
            let a = db
                .from("student")
                .select("*")
                .eq("student_id", claims.user_id.as_str())
                .execute()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let list_student: Vec<Student> = serde_json::from_str(a.as_str()).unwrap();
            if list_student.len() == 0 {
                return GeneralResponse::unauthorized(None).into_response();
            };
        }
        Role::Lecturer => {
            let a = db
                .from("lecturer")
                .select("*")
                .eq("lecturer_id", claims.user_id.as_str())
                .execute()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let list_lecturer: Vec<Lecturer> = serde_json::from_str(a.as_str()).unwrap();
            if list_lecturer.len() == 0 {
                return GeneralResponse::unauthorized(None).into_response();
            };
        }
        Role::Admin => {
            let a = db
                .from("admin")
                .select("*")
                .eq("admin_id", claims.user_id.as_str())
                .execute()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let list_admin: Vec<Admin> = serde_json::from_str(a.as_str()).unwrap();
            if list_admin.len() == 0 {
                return GeneralResponse::unauthorized(None).into_response();
            }
        }
    }
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

pub async fn student_layer<B>(
    Extension(user_claims): Extension<TokenClaims>,
    req: Request<B>,
    next: Next<B>,
) -> Response {
    if user_claims.role == Role::Student {
        next.run(req).await
    } else {
        GeneralResponse::unauthorized(None).into_response()
    }
}
pub async fn teacher_layer<B>(
    Extension(user_claims): Extension<TokenClaims>,
    req: Request<B>,
    next: Next<B>,
) -> Response {
    if user_claims.role == Role::Lecturer {
        next.run(req).await
    } else {
        GeneralResponse::unauthorized(None).into_response()
    }
}
pub async fn admin_layer<B>(
    Extension(user_claims): Extension<TokenClaims>,
    req: Request<B>,
    next: Next<B>,
) -> Response {
    if user_claims.role == Role::Admin {
        next.run(req).await
    } else {
        GeneralResponse::unauthorized(None).into_response()
    }
}

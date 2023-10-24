use std::sync::Arc;

use crate::model::database_model::{GeneralProfile, Account};
use crate::model::{ErrorMessage, GeneralResponse, TokenClaims};
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::http::Client;
use surrealdb::Surreal;
use tokio::sync::Mutex;


pub async fn profile(
    Extension(user_data): Extension<TokenClaims>,
    State(db): State<Arc<Mutex<Surreal<Client>>>>,
) -> impl IntoResponse {
    let query = format!("SELECT address, birth, email, fullname, gender, phone from {}", user_data.id);

    let profile = match db
        .lock()
        .await
        .query(query)
        .await
        .unwrap()
        .take::<Option<GeneralProfile>>(0)
    {
        Ok(result) => match result {
            Some(data) => data,
            None => {
                return GeneralResponse::not_found()
            }
        },
        Err(err) => {
            return GeneralResponse::internal_server_error(err.to_string())
        }
    };

    GeneralResponse::new(
        StatusCode::OK,
        None,
        serde_json::to_string(&profile).unwrap()
    )
}

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::model::{
    database_model::{Role, User},
    BodyMessage, GeneralResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDeleted {
    user_id: String,
}

pub async fn remove_user(
    State(db): State<Arc<Mutex<Postgrest>>>,
    Json(user): Json<UserDeleted>,
) -> impl IntoResponse {
    let identify_role = match &user.user_id[..2] {
        "ST" => Role::Student,
        "LT" => Role::Lecturer,
        _ => return GeneralResponse::bad_request("ID is invalid!".to_string()),
    };

    let table_name = identify_role.to_string().to_lowercase();
    let result_text = db
        .lock()
        .await
        .from(&table_name)
        .eq(format!("{}_id", table_name), &user.user_id)
        .delete()
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let user_deleted: Vec<User> = serde_json::from_str(&result_text).unwrap();
    if user_deleted.len() != 0 {
        GeneralResponse::body_ok(
            BodyMessage {
                code_status: StatusCode::OK.as_u16(),
                message: "Delete successfully!".to_string(),
            }
            .to_json(),
        )
    } else {
        GeneralResponse::not_found(Some("User not found!".to_string()))
    }
}

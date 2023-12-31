use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use postgrest::Postgrest;

use crate::model::{database_model::Class, DatabaseResponseError, GeneralResponse};

pub async fn update_class(
    State(db): State<Arc<Postgrest>>,
    Path(mut current_class_code): Path<String>,
    Json(mut update_class): Json<Class>,
) -> impl IntoResponse {
    current_class_code = current_class_code.to_uppercase();
    update_class.class_code = match update_class.class_code {
        Some(code) => Some(code.trim().to_uppercase()),
        None => None,
    };

    let db_response = db
        .from("class")
        .eq("class_code", current_class_code)
        .update(serde_json::to_string(&update_class).unwrap())
        .execute()
        .await
        .unwrap();
    let result_status = db_response.status();
    let result_text = db_response.text().await.unwrap();
    if result_status.is_success() {
        let updated_class: Vec<Class> = match serde_json::from_str(&result_text) {
            Ok(class) => class,
            Err(_) => return GeneralResponse::internal_server_error(None),
        };
        if updated_class.len() != 0 {
            GeneralResponse::ok(Some("Update class successfully!".to_string()))
        } else {
            GeneralResponse::not_found(Some("Class not found!".to_string()))
        }
    } else {
        let db_error: DatabaseResponseError = serde_json::from_str(&result_text).unwrap();

        GeneralResponse::bad_request(db_error.details)
    }
}

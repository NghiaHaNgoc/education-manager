use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::model::GeneralResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassDeleted {
    class_code: String,
}
pub async fn remove_class(
    State(db): State<Arc<Mutex<Postgrest>>>,
    Json(ClassDeleted { mut class_code }): Json<ClassDeleted>,
) -> impl IntoResponse {
    class_code = class_code.trim().to_uppercase();
    let result_text = db
        .lock()
        .await
        .from("class")
        .eq("class_code", class_code)
        .delete()
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let class_deleted: Vec<ClassDeleted> = match serde_json::from_str(&result_text) {
        Ok(class) => class,
        Err(_) => return GeneralResponse::internal_server_error(None),
    };
    if class_deleted.len() != 0 {
        GeneralResponse::ok(Some("Delete class successfully!".to_string()))
    } else {
        GeneralResponse::not_found(Some("Class not found!".to_string()))
    }
}

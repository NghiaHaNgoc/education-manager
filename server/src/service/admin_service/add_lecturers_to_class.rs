use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{database_model::LecturerInClass, DatabaseResponseError, GeneralResponse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddLecturersToClass {
    class: String,
    lecturers: Vec<String>,
}
pub async fn add_lecturers_to_class(
    State(db): State<Arc<Postgrest>>,
    Json(AddLecturersToClass {
        mut class,
        lecturers,
    }): Json<AddLecturersToClass>,
) -> impl IntoResponse {
    class = class.trim().to_uppercase();
    let mut lecturer_in_class_list = Vec::new();
    for lecturer in lecturers {
        lecturer_in_class_list.push(LecturerInClass {
            class: class.clone(),
            lecturer: lecturer.trim().to_uppercase(),
        })
    }
    let lecturer_in_class_json = serde_json::to_string(&lecturer_in_class_list).unwrap();
    let db_response = db
        .from("lecturer_in_class")
        .insert(lecturer_in_class_json)
        .execute()
        .await
        .unwrap();
    let status_response = db_response.status();
    let body_text = db_response.text().await.unwrap();
    if status_response.is_success() {
        GeneralResponse::ok(Some("Add lecturers to class successfully!".to_string()))
    } else {
        let response_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::bad_request(response_error.details)
    }
}

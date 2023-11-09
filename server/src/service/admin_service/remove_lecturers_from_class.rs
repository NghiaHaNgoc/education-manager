use crate::model::database_model::LecturerInClass;
use crate::model::{DatabaseResponseError, GeneralResponse};
use axum::{extract::State, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveLecturerFromClass {
    class: String,
    lecturers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseRemovedLecturer {
    class: String,
    removed_lecturers: Vec<String>,
}
pub async fn remove_lecturers_from_class(
    State(db): State<Arc<Postgrest>>,
    Json(mut remove_lecturers): Json<RemoveLecturerFromClass>,
) -> impl IntoResponse {
    remove_lecturers.class = remove_lecturers.class.trim().to_string();
    for lecturer in remove_lecturers.lecturers.iter_mut() {
        *lecturer = format!("lecturer.eq.{}", lecturer.trim().to_uppercase());
    }
    let delete_query = remove_lecturers.lecturers.join(", ");
    let db_response = db
        .from("lecturer_in_class")
        .eq("class", &remove_lecturers.class)
        .or(delete_query)
        .delete()
        .execute()
        .await
        .unwrap();
    let db_status = db_response.status();
    let body_text = db_response.text().await.unwrap();
    if db_status.is_success() {
        let deleted_lecturer: Vec<LecturerInClass> = serde_json::from_str(&body_text).unwrap();
        let deleted_lecturer_id: Vec<String> =
            deleted_lecturer.into_iter().map(|x| x.lecturer).collect();
        let response_removed = ResponseRemovedLecturer {
            class: remove_lecturers.class,
            removed_lecturers: deleted_lecturer_id,
        };
        GeneralResponse::body_ok(serde_json::to_string(&response_removed).unwrap())
    } else {
        let response_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::internal_server_error(Some(response_error.details))
    }
}

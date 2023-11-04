use crate::model::database_model::StudentInClass;
use crate::model::{DatabaseResponseError, GeneralResponse};
use axum::{extract::State, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveStudentFromClass {
    class: String,
    students: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseRemovedStudent {
    class: String,
    removed_students: Vec<String>,
}
pub async fn remove_students_from_class(
    State(db): State<Arc<Postgrest>>,
    Json(mut remove_students): Json<RemoveStudentFromClass>,
) -> impl IntoResponse {
    remove_students.class = remove_students.class.trim().to_string();
    for student in remove_students.students.iter_mut() {
        *student = format!("student.eq.{}", student.trim().to_uppercase());
    }
    let delete_query = remove_students.students.join(", ");
    let db_response = db
        .from("student_in_class")
        .eq("class", &remove_students.class)
        .or(delete_query)
        .delete()
        .execute()
        .await
        .unwrap();
    let db_status = db_response.status();
    let body_text = db_response.text().await.unwrap();
    if db_status.is_success() {
        let deleted_student: Vec<StudentInClass> = serde_json::from_str(&body_text).unwrap();
        let deleted_student_id: Vec<String> =
            deleted_student.into_iter().map(|x| x.student).collect();
        let response_removed = ResponseRemovedStudent {
            class: remove_students.class,
            removed_students: deleted_student_id,
        };
        GeneralResponse::body_ok(serde_json::to_string(&response_removed).unwrap())
    } else {
        let response_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::internal_server_error(Some(response_error.details))
    }
}

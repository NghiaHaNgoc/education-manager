use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{database_model::StudentInClass, DatabaseResponseError, GeneralResponse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddStudentsToClass {
    class: String,
    students: Vec<String>,
}
pub async fn add_students_to_class(
    State(db): State<Arc<Postgrest>>,
    Json(AddStudentsToClass {
        mut class,
        students,
    }): Json<AddStudentsToClass>,
) -> impl IntoResponse {
    class = class.trim().to_uppercase();
    let mut student_in_class_list = Vec::new();
    for student in students {
        student_in_class_list.push(StudentInClass {
            class: class.clone(),
            student: student.trim().to_uppercase(),
        })
    }
    let student_in_class_json = serde_json::to_string(&student_in_class_list).unwrap();
    let db_response = db
        .from("student_in_class")
        .insert(student_in_class_json)
        .execute()
        .await
        .unwrap();
    let status_response = db_response.status();
    let body_text = db_response.text().await.unwrap();
    if status_response.is_success() {
        GeneralResponse::ok(Some("Add students to class successfully!".to_string()))
    } else {
        let response_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::bad_request(response_error.details)
    }
}

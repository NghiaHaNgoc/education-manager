use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{
    DatabaseResponseError, GeneralResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassDetail {
    class_code: Option<String>,
    description: Option<String>,
    student_in_class: Vec<StudentInClass>,
    lecturer_in_class: Vec<LecturerInClass>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentInClass {
    student: Option<Student>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LecturerInClass {
    lecturer: Option<Lecturer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
    student_id: String,
    full_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lecturer {
    lecturer_id: String,
    full_name: Option<String>
}

pub async fn class_detail(
    State(db): State<Arc<Postgrest>>,
    Path(mut current_class_code): Path<String>,
) -> impl IntoResponse {
    current_class_code = current_class_code.to_uppercase();

    let db_response = db.from("class").select("class_code, description, student_in_class(student(student_id, full_name)), lecturer_in_class(lecturer(lecturer_id, full_name))").eq("class_code", current_class_code).execute().await.unwrap();
    let response_status = db_response.status();
    let body_text = db_response.text().await.unwrap();
    println!("{}", body_text);
    if response_status.is_success() {
        let db_class: Vec<ClassDetail> = serde_json::from_str(&body_text).unwrap();
        if db_class.len() != 0 {
            GeneralResponse::body_ok(serde_json::to_string(db_class.get(0).unwrap()).unwrap())
        } else {
            GeneralResponse::not_found(Some("Class not found!".to_string()))
        }
    } else {
        let db_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::internal_server_error(Some(db_error.details))
    }
}

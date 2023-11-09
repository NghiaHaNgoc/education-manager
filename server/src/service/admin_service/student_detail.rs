use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database_model::{Class, Gender},
    DatabaseResponseError, GeneralResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentDetail {
    student_id: Option<String>,
    full_name: Option<String>,
    birth: Option<String>,
    gender: Option<Gender>,
    address: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    student_in_class: Option<StudentInClass>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentInClass {
    class: Option<Class>,
}

pub async fn student_detail(
    State(db): State<Arc<Postgrest>>,
    Path(mut student_id): Path<String>,
) -> impl IntoResponse {
    student_id = student_id.to_uppercase();
    let db_response = db.from("student").select("student_id, full_name, birth, gender, address, email, phone, student_in_class(class(class_code, description))")
        .eq("student_id", student_id)
        .execute().await.unwrap();
    let response_status = db_response.status();
    let body_text = db_response.text().await.unwrap();
    if response_status.is_success() {
        let student: Vec<StudentDetail> = serde_json::from_str(&body_text).unwrap();
        if student.len() != 0 {
            GeneralResponse::body_ok(serde_json::to_string(student.get(0).unwrap()).unwrap())
        } else {
            GeneralResponse::not_found(Some("Student not found!".to_string()))
        }
    } else {
        let response_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::internal_server_error(Some(response_error.details))
    }
}

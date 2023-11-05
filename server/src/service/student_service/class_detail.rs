use std::sync::Arc;

use axum::{Extension, extract::{State, Path}, response::IntoResponse};
use postgrest::Postgrest;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use crate::model::{TokenClaims, database_model::{Student, Lecturer}, GeneralResponse, DatabaseResponseError};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassDetail {
    class_code: Option<String>,
    description: Option<String>,
    student_in_class: Vec<StudentInClass>,
    lecturer_in_class: Vec<LecturerInClass>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentInClass {
    student: Option<Student>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LecturerInClass {
    lecturer: Option<Lecturer>,
}

pub async fn class_detail(
    Extension(user_data): Extension<TokenClaims>,
    State(db): State<Arc<Postgrest>>,
    Path(mut current_class_code): Path<String>,
    ) -> impl IntoResponse {
    current_class_code = current_class_code.to_uppercase();

    let db_response = db.from("class").select("class_code, description, student_in_class(student(student_id, full_name)), lecturer_in_class(lecturer(lecturer_id, full_name))").eq("class_code", current_class_code).cs(user_data.user_id,"student_in_class.student.student_id").execute().await.unwrap();
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

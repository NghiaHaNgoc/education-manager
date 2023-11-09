use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    DatabaseResponseError, GeneralResponse, TokenClaims,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassDetail {
    class_code: Option<String>,
    description: Option<String>,
    student_in_class: Vec<StudentInClass>,
    lecturer_in_class: Vec<LecturerInClass>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentInClass {
    student: Student,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LecturerInClass {
    lecturer: Lecturer,
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
    Extension(user_data): Extension<TokenClaims>,
    State(db): State<Arc<Postgrest>>,
    Path(mut current_class_code): Path<String>,
) -> impl IntoResponse {
    current_class_code = current_class_code.to_uppercase();

    let db_response = db.from("class").select("class_code, description, student_in_class(student(student_id, full_name)), lecturer_in_class(lecturer(lecturer_id, full_name))").eq("class_code", current_class_code).execute().await.unwrap();
    let response_status = db_response.status();
    let body_text = db_response.text().await.unwrap();
    println!("{}", body_text);
    if response_status.is_success() {
        let mut db_class: Vec<ClassDetail> = serde_json::from_str(&body_text).unwrap();
        db_class = db_class
            .into_iter()
            .filter(|x| {
                let user_id = &user_data.user_id;
                for x in &x.student_in_class {
                    if x.student.student_id.eq(user_id) {
                        return true;
                    }
                }
                false
            })
            .collect();
        if db_class.len() != 0 {
            GeneralResponse::body_ok(serde_json::to_string(db_class.get(0).unwrap()).unwrap())
        } else {
            GeneralResponse::not_found(Some("This is not class you assigned!".to_string()))
        }
    } else {
        let db_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::internal_server_error(Some(db_error.details))
    }
}

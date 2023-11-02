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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LecturerDetail {
    lecturer_id: Option<String>,
    full_name: Option<String>,
    birth: Option<String>,
    gender: Option<Gender>,
    address: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    lecturer_in_class: Vec<LecturerInClass>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LecturerInClass {
    class: Option<Class>,
}

pub async fn lecturer_detail(
    State(db): State<Arc<Postgrest>>,
    Path(mut lecturer_id): Path<String>,
) -> impl IntoResponse {
    lecturer_id = lecturer_id.to_uppercase();
    let db_response = db.from("lecturer").select("lecturer_id, full_name, birth, gender, address, email, phone, lecturer_in_class(class(class_code, description))")
        .eq("lecturer_id", lecturer_id)
        .execute().await.unwrap();
    let response_status = db_response.status();
    let body_text = db_response.text().await.unwrap();
    if response_status.is_success() {
        let lecturer: Vec<LecturerDetail> = serde_json::from_str(&body_text).unwrap();
        if lecturer.len() != 0 {
            GeneralResponse::body_ok(serde_json::to_string(lecturer.get(0).unwrap()).unwrap())
        } else {
            GeneralResponse::not_found(Some("Lecturer not found!".to_string()))
        }
    } else {
        let response_error: DatabaseResponseError = serde_json::from_str(&body_text).unwrap();
        GeneralResponse::internal_server_error(Some(response_error.details))
    }
}

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::model::{database_model::Student, GeneralResponse};

#[derive(Serialize, Deserialize)]
struct StudentListResponse {
    student_list: Vec<Student>,
    range: String,
    total: u32,
}
#[derive(Serialize, Deserialize)]
struct Count {
    count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct QueryOptions {
    page_number: Option<u32>,
    students_per_page: Option<u32>,
}

pub async fn list_student(
    State(db): State<Arc<Mutex<Postgrest>>>,
    Query(QueryOptions {
        page_number,
        students_per_page,
    }): Query<QueryOptions>,
) -> impl IntoResponse {
    let page_number = page_number.unwrap_or(1);
    let students_per_page = students_per_page.unwrap_or(24);
    let from_index = (page_number - 1) * students_per_page;
    let to_index = from_index + students_per_page - 1;

    let student_query = db
        .lock()
        .await
        .from("student")
        .select("student_id, full_name, birth, gender, address, email, phone")
        .exact_count()
        .range(from_index as usize, to_index as usize)
        .order("student_id.asc")
        .execute()
        .await
        .unwrap();
    let header: &HeaderMap = student_query.headers();
    let (range, total) = get_range_and_total(header);
    let body = student_query.text().await.unwrap();
    let student_list: Vec<Student> = serde_json::from_str(&body).unwrap_or(Vec::new());
    let student_list_response = StudentListResponse {
        student_list,
        range,
        total,
    };

    GeneralResponse::body_ok(serde_json::to_string(&student_list_response).unwrap())
}

fn get_range_and_total(header: &HeaderMap) -> (String, u32) {
    let mut content_range = header
        .get("content-range")
        .unwrap()
        .to_str()
        .unwrap()
        .split("/");
    let range = content_range.next().unwrap().to_string();
    let total: u32 = content_range.next().unwrap().parse().unwrap();
    (range, total)
}

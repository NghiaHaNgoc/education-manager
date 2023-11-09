
use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{GeneralResponse, database_model::{Gender, LecturerInClass}};

#[derive(Serialize, Deserialize)]
struct LecturerListResponse {
    lecturer_list: Vec<Lecturer>,
    // range: String,
    // total: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lecturer {
    pub lecturer_id: Option<String>,
    pub full_name: Option<String>,
    pub birth: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(skip_serializing)]
    pub lecturer_in_class: Vec<LecturerInClass>
}

#[derive(Serialize, Deserialize)]
struct Count {
    count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct QueryOptions {
    page_number: Option<u32>,
    lecturers_per_page: Option<u32>,
}

pub async fn lecturer_not_in_any_class(
    State(db): State<Arc<Postgrest>>
    // Query(QueryOptions {
    //     page_number,
    //     lecturers_per_page,
    // }): Query<QueryOptions>,
) -> impl IntoResponse {
    // let page_number = page_number.unwrap_or(1);
    // let lecturers_per_page = lecturers_per_page.unwrap_or(24);
    // let from_index = (page_number - 1) * lecturers_per_page;
    // let to_index = from_index + lecturers_per_page - 1;

    let lecturer_query = db
        .from("lecturer")
        .select("lecturer_id, full_name, birth, gender, address, email, phone, lecturer_in_class(lecturer, class)")
        // .exact_count()
        // .range(from_index as usize, to_index as usize)
        .order("lecturer_id.asc")
        .execute()
        .await
        .unwrap();
    // let header: &HeaderMap = lecturer_query.headers();
    // let (range, total) = get_range_and_total(header);
    let body = lecturer_query.text().await.unwrap();
    let mut lecturer_list: Vec<Lecturer> = serde_json::from_str(&body).unwrap_or(Vec::new());
    lecturer_list = lecturer_list.into_iter().filter(|lecturer| lecturer.lecturer_in_class.is_empty()).collect();
    let lecturer_list_response = LecturerListResponse {
        lecturer_list,
        // range,
        // total,
    };

    GeneralResponse::body_ok(serde_json::to_string(&lecturer_list_response).unwrap())
}

// fn get_range_and_total(header: &HeaderMap) -> (String, u32) {
//     let mut content_range = header
//         .get("content-range")
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .split("/");
//     let range = content_range.next().unwrap().to_string();
//     let total: u32 = content_range.next().unwrap().parse().unwrap();
//     (range, total)
// }

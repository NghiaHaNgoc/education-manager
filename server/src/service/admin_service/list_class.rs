use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{database_model::Class, GeneralResponse};

#[derive(Serialize, Deserialize)]
struct ClassListResponse {
    class_list: Vec<Class>,
    range: String,
    total: u32,
}

#[derive(Serialize, Deserialize)]
pub struct QueryOptions {
    page_number: Option<u32>,
    classes_per_page: Option<u32>,
}

pub async fn list_class(
    State(db): State<Arc<Postgrest>>,
    Query(QueryOptions {
        page_number,
        classes_per_page,
    }): Query<QueryOptions>,
) -> impl IntoResponse {
    let page_number = page_number.unwrap_or(1);
    let classes_per_page = classes_per_page.unwrap_or(24);
    let from_index = (page_number - 1) * classes_per_page;
    let to_index = from_index + classes_per_page - 1;

    let class_query = db
        .from("class")
        .select("class_code, description")
        .exact_count()
        .range(from_index as usize, to_index as usize)
        .order("class_code.asc")
        .execute()
        .await
        .unwrap();
    let header: &HeaderMap = class_query.headers();
    let (range, total) = get_range_and_total(header);
    let body = class_query.text().await.unwrap();
    let class_list: Vec<Class> = serde_json::from_str(&body).unwrap_or(Vec::new());
    let class_list_response = ClassListResponse {
        class_list,
        range,
        total,
    };

    GeneralResponse::body_ok(serde_json::to_string(&class_list_response).unwrap())
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

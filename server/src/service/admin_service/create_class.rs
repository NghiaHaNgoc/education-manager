use std::sync::Arc;

use axum::{extract::State, Json, response::IntoResponse};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tokio::sync::Mutex;

use crate::model::{GeneralResponse, DatabaseResponseError};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewClass {
    class_code: String,
    description: Option<String>
}

pub async fn create_class(

    State(db): State<Arc<Mutex<Postgrest>>>,
    Json(mut new_class): Json<NewClass>,
) -> impl IntoResponse {
    new_class.class_code = new_class.class_code.trim().to_uppercase();

    if new_class.class_code.is_empty() {
        return GeneralResponse::bad_request("Class code is empty!".to_string());
    }
    let response = db.lock().await.from("class").insert(serde_json::to_string(&new_class).unwrap()).execute().await.unwrap();
    if response.status().is_success() {
        GeneralResponse::ok(Some("Create class successfully!".to_string()))
    } else {
        let database_response: DatabaseResponseError = serde_json::from_str(&response.text().await.unwrap()).unwrap();
        GeneralResponse::bad_request(database_response.details)
    }

    
}

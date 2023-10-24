use std::sync::{Arc, Mutex};

use axum::{extract::State, response::IntoResponse};
use serde::{Serialize, Deserialize};
use surrealdb::{Surreal, engine::remote::http::Client};

use crate::model::database_model::StudentProfile;

#[derive(Serialize, Deserialize)]
struct ListStudent {
    list_student: Vec<StudentProfile>,
    total: u32
}

fn list_student(
    State(db): State<Arc<Mutex<Surreal<Client>>>>,
    
) -> impl IntoResponse {
    let query = format!("");
}

use std::sync::Arc;

use crate::model::database_model::{Role, Student};
use crate::model::{GeneralResponse, TokenClaims};
use axum::extract::{Extension, State};
use axum::response::IntoResponse;
use postgrest::Postgrest;
use tokio::sync::Mutex;

pub async fn profile(
    Extension(user_data): Extension<TokenClaims>,
    State(db): State<Arc<Mutex<Postgrest>>>,
) -> impl IntoResponse {
    match user_data.role {
        Role::Student => get_student_profile(user_data, db).await,
        Role::Lecturer => get_lecturer_profile(user_data, db).await,
        Role::Admin => get_admin_profile(user_data, db).await,
    }
}
async fn get_student_profile(user_data: TokenClaims, db: Arc<Mutex<Postgrest>>) -> GeneralResponse {
    let text_data = db
        .lock()
        .await
        .from("student")
        .select("student_id, full_name, birth, gender, address, email, phone")
        .eq("student_id", user_data.user_id)
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut student_list: Vec<Student> = serde_json::from_str(&text_data).unwrap();
    if student_list.len() != 0 {
        let student = student_list.remove(0);
        let student_json = serde_json::to_string(&student).unwrap();
        GeneralResponse::ok(student_json)
    } else {
        GeneralResponse::not_found(Some(String::from("Profile not found!")))
    }
}

async fn get_lecturer_profile(
    user_data: TokenClaims,
    db: Arc<Mutex<Postgrest>>,
) -> GeneralResponse {
    let text_data = db
        .lock()
        .await
        .from("lecturer")
        .select("lecturer_id, full_name, birth, gender, address, email, phone")
        .eq("lecturer_id", user_data.user_id)
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut lecturer_list: Vec<Student> = serde_json::from_str(&text_data).unwrap();
    if lecturer_list.len() != 0 {
        let lecturer = lecturer_list.remove(0);
        let lecturer_json = serde_json::to_string(&lecturer).unwrap();
        GeneralResponse::ok(lecturer_json)
    } else {
        GeneralResponse::not_found(Some(String::from("Profile not found!")))
    }
}

async fn get_admin_profile(user_data: TokenClaims, db: Arc<Mutex<Postgrest>>) -> GeneralResponse {
    let text_data = db
        .lock()
        .await
        .from("admin")
        .select("admin_id, full_name, birth, gender, address, email, phone")
        .eq("admin_id", user_data.user_id)
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut admin_list: Vec<Student> = serde_json::from_str(&text_data).unwrap();
    if admin_list.len() != 0 {
        let admin = admin_list.remove(0);
        let admin_json = serde_json::to_string(&admin).unwrap();
        GeneralResponse::ok(admin_json)
    } else {
        GeneralResponse::not_found(Some(String::from("Profile not found!")))
    }
}

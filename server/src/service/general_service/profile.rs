use std::sync::Arc;

use crate::model::database_model::{Role, Student, Gender, Class};
use crate::model::{GeneralResponse, TokenClaims};
use axum::extract::{Extension, State};
use axum::response::IntoResponse;
use postgrest::Postgrest;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use tokio::sync::Mutex;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentProfile {
    student_id: Option<String>,
    full_name: Option<String>,
    birth: Option<String>,
    gender: Option<Gender>,
    address: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    student_in_class: Option<Vec<InClassProfile>>
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InClassProfile {
    class: Option<Class>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LecturerProfile {
    pub lecturer_id: Option<String>,
    pub full_name: Option<String>,
    pub birth: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    lecturer_in_class: Option<Vec<InClassProfile>>
}

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
        .select("student_id, full_name, birth, gender, address, email, phone, student_in_class(class(class_code, description))")
        .eq("student_id", user_data.user_id)
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut student_list: Vec<StudentProfile> = serde_json::from_str(&text_data).unwrap();
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
        .select("lecturer_id, full_name, birth, gender, address, email, phone, lecturer_in_class(class(class_code, description))")
        .eq("lecturer_id", user_data.user_id)
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut lecturer_list: Vec<LecturerProfile> = serde_json::from_str(&text_data).unwrap();
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

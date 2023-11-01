use std::sync::Arc;

use crate::model::database_model::{Class, Gender, Role, Student};
use crate::model::{GeneralResponse, TokenClaims};
use axum::extract::{Extension, State};
use axum::response::IntoResponse;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tokio::sync::Mutex;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    user_id: Option<String>,
    full_name: Option<String>,
    birth: Option<String>,
    gender: Option<Gender>,
    address: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    in_class: Option<Vec<InClassProfile>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InClassProfile {
    class: Option<Class>,
}

pub async fn profile(
    Extension(user_data): Extension<TokenClaims>,
    State(db): State<Arc<Mutex<Postgrest>>>,
) -> impl IntoResponse {
    match user_data.role {
        Role::Admin => get_admin_profile(user_data, db).await,
        _ => get_student_and_lecturer_profile(user_data, db).await,
    }
}
async fn get_student_and_lecturer_profile(
    user_data: TokenClaims,
    db: Arc<Mutex<Postgrest>>,
) -> GeneralResponse {
    let table_name = user_data.role.to_string().to_lowercase();
    let text_data = db
        .lock()
        .await
        .from(&table_name)
        .select(format!("user_id:{}_id, full_name, birth, gender, address, email, phone, in_class:{}_in_class(class(class_code, description))", table_name, table_name))
        .eq(format!("{}_id", table_name), user_data.user_id)
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut user_list: Vec<Profile> = serde_json::from_str(&text_data).unwrap();
    if user_list.len() != 0 {
        let user = user_list.remove(0);
        let user_json = serde_json::to_string(&user).unwrap();
        GeneralResponse::ok(user_json)
    } else {
        GeneralResponse::not_found(Some(String::from("Profile not found!")))
    }
}

async fn get_admin_profile(user_data: TokenClaims, db: Arc<Mutex<Postgrest>>) -> GeneralResponse {
    let text_data = db
        .lock()
        .await
        .from("admin")
        .select("user_id:admin_id, full_name, birth, gender, address, email, phone")
        .eq("admin_id", user_data.user_id)
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut admin_list: Vec<Profile> = serde_json::from_str(&text_data).unwrap();
    if admin_list.len() != 0 {
        let admin = admin_list.remove(0);
        let admin_json = serde_json::to_string(&admin).unwrap();
        GeneralResponse::ok(admin_json)
    } else {
        GeneralResponse::not_found(Some(String::from("Profile not found!")))
    }
}

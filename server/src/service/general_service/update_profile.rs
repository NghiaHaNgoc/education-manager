use std::sync::Arc;

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tokio::sync::Mutex;

use crate::{model::{database_model::Gender, BodyMessage, GeneralResponse, TokenClaims}, service::{validate_email_and_phone, validate_email, validate_phone}};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProfileOption {
    full_name: Option<String>,
    birth: Option<String>,
    gender: Option<Gender>,
    address: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    password: Option<String>,
}


pub async fn update_profile(
    Extension(user_data): Extension<TokenClaims>,
    State(db): State<Arc<Mutex<Postgrest>>>,
    Json(update_option): Json<UpdateProfileOption>,
) -> impl IntoResponse {
    if let Some(response) = validate_info_update(&user_data, &db, &update_option).await {
        return response;
    }
    update_user_profile(user_data, db, update_option).await;
    return GeneralResponse::ok(
        BodyMessage {
            code_status: StatusCode::OK.as_u16(),
            message: "Update profile successfully!".to_string(),
        }
        .to_json(),
    );
}

async fn validate_info_update(
    user_data: &TokenClaims,
    db: &Arc<Mutex<Postgrest>>,
    update_option: &UpdateProfileOption,
) -> Option<GeneralResponse> {

    // NOTE: validate email & phone
    if let (Some(email), Some(phone)) = (update_option.email.as_ref(), update_option.phone.as_ref()) {
        validate_email_and_phone(user_data, db, &email, &phone).await
    } else {
        if let Some(email) = update_option.email.as_ref() {
            return validate_email(user_data, db, email).await;
        }

        if let Some(phone) = update_option.phone.as_ref() {
            return validate_phone(user_data, db, phone).await;
        }
        None
    }
}


async fn update_user_profile(
    user_data: TokenClaims,
    db: Arc<Mutex<Postgrest>>,
    update_option: UpdateProfileOption,
) {
    let table_name = user_data.role.to_string().to_lowercase();
    let result_update = db
        .lock()
        .await
        .from(&table_name)
        .eq(format!("{}_id", table_name), user_data.user_id)
        .update(serde_json::to_string(&update_option).unwrap())
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("Update: {}", result_update);
}

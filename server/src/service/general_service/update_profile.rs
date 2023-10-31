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

use crate::model::{database_model::Gender, BodyMessage, GeneralResponse, TokenClaims};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailOrPhone {
    email: Option<String>,
    phone: Option<String>,
}

struct DuplicateEmailOrPhone {
    duplicated_email: bool,
    duplicated_phone: bool,
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
        // NOTE: validate phone
        if let Some(email) = update_option.email.as_ref() {
            return validate_email(user_data, db, email).await;
        }

        if let Some(phone) = update_option.phone.as_ref() {
            return validate_phone(user_data, db, phone).await;
        }
        None
    }
}

async fn validate_email_and_phone(
    user_data: &TokenClaims,
    db: &Arc<Mutex<Postgrest>>,
    email: &String,
    phone: &String,
) -> Option<GeneralResponse> {
    let mut duplicated_field = DuplicateEmailOrPhone {
        duplicated_email: false,
        duplicated_phone: false,
    };
    let duplicated_email_or_phone_student_query = db
        .lock()
        .await
        .from("student")
        .select("email, phone")
        .or(format!("email.eq.{}, phone.eq.{}", email, phone))
        .neq("student_id", user_data.user_id.as_str())
        .execute();
    let duplicated_email_or_phone_lecturer_query = db
        .lock()
        .await
        .from("lecturer")
        .select("email, phone")
        .or(format!("email.eq.{}, phone.eq.{}", email, phone))
        .neq("lecturer_id", user_data.user_id.as_str())
        .execute();
    let duplicated_email_or_phone_admin_query = db
        .lock()
        .await
        .from("admin")
        .select("email, phone")
        .neq("admin_id", user_data.user_id.as_str())
        .or(format!("email.eq.{}, phone.eq.{}", email, phone))
        .execute();
    let db_email_or_phone_student: Vec<EmailOrPhone> = serde_json::from_str(
        duplicated_email_or_phone_student_query
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .as_str(),
    )
    .unwrap();
    if let Some(response) = validate_email_and_phone_list(
        db_email_or_phone_student,
        &mut duplicated_field,
        email,
        phone,
    ) {
        return Some(response);
    }

    let db_email_or_phone_lecturer: Vec<EmailOrPhone> = serde_json::from_str(
        duplicated_email_or_phone_lecturer_query
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .as_str(),
    )
    .unwrap();
    if let Some(response) = validate_email_and_phone_list(
        db_email_or_phone_lecturer,
        &mut duplicated_field,
        email,
        phone,
    ) {
        return Some(response);
    }

    let db_email_or_phone_admin: Vec<EmailOrPhone> = serde_json::from_str(
        duplicated_email_or_phone_admin_query
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .as_str(),
    )
    .unwrap();
    if let Some(response) =
        validate_email_and_phone_list(db_email_or_phone_admin, &mut duplicated_field, email, phone)
    {
        return Some(response);
    }

    if duplicated_field.duplicated_email && duplicated_field.duplicated_phone {
        return Some(GeneralResponse::bad_request(
            "Email and phone are duplicated!".to_string(),
        ));
    } else {
        if duplicated_field.duplicated_email {
            return Some(GeneralResponse::bad_request(
                "Email is duplicated!".to_string(),
            ));
        }
        if duplicated_field.duplicated_phone {
            return Some(GeneralResponse::bad_request(
                "Phone is duplicated!".to_string(),
            ));
        }
        return None;
    }
}

async fn validate_email(
    user_data: &TokenClaims,
    db: &Arc<Mutex<Postgrest>>,
    email: &String,
) -> Option<GeneralResponse> {
    let error_response = Some(GeneralResponse::bad_request(
                    "Email is duplicated!".to_string(),
                ));
    
            let duplicated_email_student_query = db
                .lock()
                .await
                .from("student")
                .select("email")
                .eq("email", email)
        .neq("student_id", user_data.user_id.as_str())
                .execute();
            let duplicated_email_lecturer_query = db
                .lock()
                .await
                .from("lecturer")
                .select("email")
                .eq("email", email)
        .neq("lecturer_id", user_data.user_id.as_str())
                .execute();
            let duplicated_email_admin_query = db
                .lock()
                .await
                .from("admin")
                .select("email")
                .eq("email", email)
        .neq("admin_id", user_data.user_id.as_str())
                .execute();

            let db_email_or_phone_student: Vec<EmailOrPhone> = serde_json::from_str(
                duplicated_email_student_query
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .as_str(),
            )
            .unwrap();
            if db_email_or_phone_student.len() != 0 {
                return error_response;
            }

            let db_email_or_phone_lecturer: Vec<EmailOrPhone> = serde_json::from_str(
                duplicated_email_lecturer_query
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

            if db_email_or_phone_lecturer.len() != 0 {
                return error_response;
            }
            let db_email_or_phone_admin: Vec<EmailOrPhone> = serde_json::from_str(
                duplicated_email_admin_query
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

            if db_email_or_phone_admin.len() != 0 {
        return error_response;
            }
            return None;
}


async fn validate_phone(
    user_data: &TokenClaims,
    db: &Arc<Mutex<Postgrest>>,
    phone: &String,
) -> Option<GeneralResponse> {
    
    let error_response = Some(GeneralResponse::bad_request(
                    "Phone is duplicated!".to_string(),
                ));
            let duplicated_phone_student_query = db
                .lock()
                .await
                .from("student")
                .select("phone")
                .eq("phone", phone)
        .neq("student_id", user_data.user_id.as_str())
                .execute();
            let duplicated_phone_lecturer_query = db
                .lock()
                .await
                .from("lecturer")
                .select("phone")
                .eq("phone", phone)
        .neq("lecturer_id", user_data.user_id.as_str())
                .execute();
            let duplicated_phone_admin_query = db
                .lock()
                .await
                .from("admin")
                .select("phone")
                .eq("phone", phone)
        .neq("admin_id", user_data.user_id.as_str())
                .execute();

            let db_email_or_phone_student: Vec<EmailOrPhone> = serde_json::from_str(
                duplicated_phone_student_query
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .as_str(),
            )
            .unwrap();
            if db_email_or_phone_student.len() != 0 {
                return error_response;
            }

            let db_email_or_phone_lecturer: Vec<EmailOrPhone> = serde_json::from_str(
                duplicated_phone_lecturer_query
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

            if db_email_or_phone_lecturer.len() != 0 {
                return error_response;
            }
            let db_email_or_phone_admin: Vec<EmailOrPhone> = serde_json::from_str(
                duplicated_phone_admin_query
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

            if db_email_or_phone_admin.len() != 0 {
                return error_response;
            }
            return None;
}

fn validate_email_and_phone_list(
    db_email_or_phone: Vec<EmailOrPhone>,
    duplicated_field: &mut DuplicateEmailOrPhone,
    email: &String,
    phone: &String,
) -> Option<GeneralResponse> {
    for i in db_email_or_phone {
        if let Some(db_email) = i.email {
            if db_email.eq(email) {
                duplicated_field.duplicated_email = true;
            }
        }
        if let Some(db_phone) = i.phone {
            if db_phone.eq(phone) {
                duplicated_field.duplicated_phone = true;
            }
        }

        if duplicated_field.duplicated_email && duplicated_field.duplicated_phone {
            return Some(GeneralResponse::bad_request(
                "Email and phone are duplicated!".to_string(),
            ));
        }
    }
    return None;
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

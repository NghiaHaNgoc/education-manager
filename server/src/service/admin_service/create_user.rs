use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tokio::sync::Mutex;

use crate::model::{
    database_model::{Gender, Lecturer, Role, Student},
    GeneralResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    #[serde(skip_serializing)]
    pub role: Role,
    pub full_name: String,
    pub birth: String,
    pub gender: Gender,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}
pub async fn create_user(
    State(db): State<Arc<Mutex<Postgrest>>>,
    Json(new_user): Json<NewUser>,
) -> impl IntoResponse {
    if let Some(response) = validate_info_create(&db, &new_user).await {
        return response;
    }

    if new_user.role == Role::Admin {
        return GeneralResponse::bad_request("Invalid role!".to_string());
    }

    let table_name = new_user.role.to_string().to_lowercase();
    println!("role: {}", table_name);
    let text_result = db
        .lock()
        .await
        .from(&table_name)
        .insert(format!("[{}]", serde_json::to_string(&new_user).unwrap()))
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", text_result);
    match new_user.role {
        Role::Student => {
            let mut student: Vec<Student> = match serde_json::from_str(&text_result) {
                Ok(student) => student,
                Err(_) => return GeneralResponse::internal_server_error(None),
            };
            GeneralResponse::body_ok(serde_json::to_string(&student.remove(0)).unwrap())
        }
        Role::Lecturer => {
            let mut lecturer: Vec<Lecturer> = match serde_json::from_str(&text_result) {
                Ok(lecturer) => lecturer,
                Err(_) => return GeneralResponse::internal_server_error(None),
            };
            GeneralResponse::body_ok(serde_json::to_string(&lecturer.remove(0)).unwrap())
        }
        Role::Admin => GeneralResponse::internal_server_error(None),
    }
}
async fn validate_info_create(
    db: &Arc<Mutex<Postgrest>>,
    new_user: &NewUser,
) -> Option<GeneralResponse> {
    // NOTE: validate email & phone
    if let (Some(email), Some(phone)) = (new_user.email.as_ref(), new_user.phone.as_ref()) {
        validate_email_and_phone(db, &email, &phone).await
    } else {
        if let Some(email) = new_user.email.as_ref() {
            return validate_email(db, email).await;
        }

        if let Some(phone) = new_user.phone.as_ref() {
            return validate_phone(db, phone).await;
        }
        None
    }
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

async fn validate_email_and_phone(
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
        .execute();
    let duplicated_email_or_phone_lecturer_query = db
        .lock()
        .await
        .from("lecturer")
        .select("email, phone")
        .or(format!("email.eq.{}, phone.eq.{}", email, phone))
        .execute();
    let duplicated_email_or_phone_admin_query = db
        .lock()
        .await
        .from("admin")
        .select("email, phone")
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

async fn validate_email(db: &Arc<Mutex<Postgrest>>, email: &String) -> Option<GeneralResponse> {
    let error_response = Some(GeneralResponse::bad_request(
        "Email is duplicated!".to_string(),
    ));

    let duplicated_email_student_query = db
        .lock()
        .await
        .from("student")
        .select("email")
        .eq("email", email)
        .execute();
    let duplicated_email_lecturer_query = db
        .lock()
        .await
        .from("lecturer")
        .select("email")
        .eq("email", email)
        .execute();
    let duplicated_email_admin_query = db
        .lock()
        .await
        .from("admin")
        .select("email")
        .eq("email", email)
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

async fn validate_phone(db: &Arc<Mutex<Postgrest>>, phone: &String) -> Option<GeneralResponse> {
    let error_response = Some(GeneralResponse::bad_request(
        "Phone is duplicated!".to_string(),
    ));
    let duplicated_phone_student_query = db
        .lock()
        .await
        .from("student")
        .select("phone")
        .eq("phone", phone)
        .execute();
    let duplicated_phone_lecturer_query = db
        .lock()
        .await
        .from("lecturer")
        .select("phone")
        .eq("phone", phone)
        .execute();
    let duplicated_phone_admin_query = db
        .lock()
        .await
        .from("admin")
        .select("phone")
        .eq("phone", phone)
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

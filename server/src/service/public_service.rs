use std::sync::Arc;

use axum::extract::State;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::{response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::database_model::Role;
use crate::model::{GeneralResponse, LoginData, LoginSuccess, TokenClaims, SECRECT_KEY};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CustomUser {
    user_id: Option<String>,
    role: Option<Role>,
    full_name: Option<String>
}

pub async fn login(
    State(db): State<Arc<Postgrest>>,
    Json(login_data): Json<LoginData>,
) -> impl IntoResponse {
    println!("{:?}", login_data);
    // Prepare response;
    let mut user = CustomUser {
        user_id: None,
        role: None,
        full_name: None
    };
    let mut verified = false;

    // NOTE: initialize send query to database
    let student_query = db
        .from("student")
        .select("user_id:student_id, full_name")
        .and(format!(
            "student_id.eq.{}, password.eq.{}",
            login_data.username, login_data.password
        ))
        .execute();
    let lecturer_query = db
        .from("lecturer")
        .select("user_id:lecturer_id, full_name")
        .and(format!(
            "lecturer_id.eq.{}, password.eq.{}",
            login_data.username, login_data.password
        ))
        .execute();
    let admin_query = db
        .from("admin")
        .select("user_id:admin_id, full_name")
        .and(format!(
            "admin_id.eq.{}, password.eq.{}",
            login_data.username, login_data.password
        ))
        .execute();

    // validate user
    let student_json_list = student_query.await.unwrap().text().await.unwrap();
    let mut student_list: Vec<CustomUser> =
        serde_json::from_str(student_json_list.as_str()).unwrap();
    if student_list.len() != 0 {
        user = student_list.remove(0);
        user.role = Some(Role::Student);
        verified = true;
    }

    if !verified {
        let lecturer_json_list = lecturer_query.await.unwrap().text().await.unwrap();
        let mut lecturer_list: Vec<CustomUser> =
            serde_json::from_str(lecturer_json_list.as_str()).unwrap();
        if lecturer_list.len() != 0 {
            user = lecturer_list.remove(0);
            user.role = Some(Role::Lecturer);
            verified = true;
        }
    }

    if !verified {
        let admin_json_list = admin_query.await.unwrap().text().await.unwrap();
        let mut admin_list: Vec<CustomUser> =
            serde_json::from_str(admin_json_list.as_str()).unwrap();
        if admin_list.len() != 0 {
            user = admin_list.remove(0);
            user.role = Some(Role::Admin);
            verified = true;
        }
    }

    if !verified {
        return GeneralResponse::unauthorized(Some("Login failed!".to_string()));
    }
    let CustomUser{role, user_id, full_name} = user.clone();
    let token = create_token(user);
    let cookie = create_cookie(&token);

    let mut header_map = HeaderMap::new();
    header_map.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    header_map.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    GeneralResponse::new(
        StatusCode::OK,
        Some(header_map),
        LoginSuccess::to_json(token, role, user_id, full_name),
    )
}

fn create_token(user: CustomUser) -> String {
    let user_id = user.user_id.unwrap();
    let role = user.role.unwrap();
    let now = Utc::now();
    let exp = (now + Duration::hours(12)).timestamp() as usize;

    let claims = TokenClaims { user_id, role, exp };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRECT_KEY.as_bytes()),
    )
    .unwrap();
    token
}

fn create_cookie(token: &str) -> Cookie {
    Cookie::build("TKID", token.to_string())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::None)
        .http_only(true)
        .finish()
}

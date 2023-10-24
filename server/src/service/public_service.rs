use std::sync::Arc;

use axum::extract::State;
use axum::http::{StatusCode, header, HeaderMap, HeaderValue};
use axum::response::Response;
use axum::{response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, EncodingKey};
use surrealdb::engine::remote::http::Client;
use surrealdb::Surreal;
use tokio::sync::Mutex;

use crate::model::database_model::Account;
use crate::model::{LoginData, GeneralResponse, LoginSuccess, TokenClaims, SECRECT_KEY};

pub async fn login(
    db: State<Arc<Mutex<Surreal<Client>>>>,
    Json(login_data): Json<LoginData>,
) -> impl IntoResponse {
    println!("{:?}", login_data);
    // Prepare response;

    let db_lock = db.lock().await;
    let query = format!(
        "SELECT user_profile, role from accounts where username = \"{}\" && password = \"{}\";",
        login_data.username, login_data.password
    );
    let a:Option<Account> = match db_lock.query(query).await.unwrap().take(0) {
        Ok(account) => account,
        Err(err) => {
            println!("{}", err.to_string());
            return GeneralResponse::internal_server_error(err.to_string())
        }
    };

    let account = match a {
        Some(acc) => acc,
        None => return GeneralResponse::unauthorized()
    };
    
    let role = account.role.clone().unwrap();
    let token = create_token(account);
    let cookie = create_cookie(&token);

    let mut header_map = HeaderMap::new();
    header_map.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    header_map.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());


     GeneralResponse::new(StatusCode::OK, Some(header_map), LoginSuccess::to_json(token, role))
    
}

fn create_token(account: Account) -> String {
    let rcid = account.user_profile.unwrap();
    let role = account.role.unwrap();
    let now = Utc::now();
    let exp = (now + Duration::minutes(60)).timestamp() as usize;

    let claims = TokenClaims {
        id: rcid,
        role,
        exp
    };
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

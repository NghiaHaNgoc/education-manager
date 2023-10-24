use axum::{
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::{opt::RecordId, sql::json};
use self::database_model::Role;

pub mod database_model;

pub const SECRECT_KEY: &str = "FER201m";


#[derive(Debug, Clone)]
pub struct GeneralResponse {
    pub status_code: StatusCode,
    pub header: HeaderMap,
    pub body: String,
}


// NOTE: General response for all layer and handler
impl GeneralResponse {
    pub fn new(
        status_code: StatusCode,
        header: Option<HeaderMap>,
        body: String,
    ) -> GeneralResponse {
        let header = header.unwrap_or_else(|| {
            let mut head = HeaderMap::new();
            head.append(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
            head
        });
        GeneralResponse {
            status_code,
            header,
            body,
        }
    }
    pub fn unauthorized() -> GeneralResponse{
        let mut head = HeaderMap::new();
        head.append(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
        GeneralResponse {
            status_code: StatusCode::UNAUTHORIZED,
            header: head,
            body: ErrorMessage {
                code_status: StatusCode::UNAUTHORIZED.as_u16(),
                message: String::from("Unauthorized!")
            }.to_json()
        }
    }
    pub fn internal_server_error(message: String) -> GeneralResponse {
        
        let mut head = HeaderMap::new();
        head.append(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
        GeneralResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            header: head,
            body: ErrorMessage {
                code_status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message
            }.to_json()
        }
    }
    pub fn not_found() -> GeneralResponse {
        
        let mut head = HeaderMap::new();
        head.append(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
        GeneralResponse {
            status_code: StatusCode::NOT_FOUND,
            header: head,
            body: ErrorMessage {
                code_status: StatusCode::NOT_FOUND.as_u16(),
                message: String::from("Not found!")
            }.to_json()
        }
    }
}

impl IntoResponse for GeneralResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.header, self.body).into_response()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginSuccess {
    pub login_success: bool,
    pub code_status: u16,
    pub message: String,
    pub role: Option<Role>,
    pub token: Option<String>
}

impl LoginSuccess {
    pub fn to_json(token: String, role: Role) -> String{
        serde_json::to_string(&LoginSuccess {
            login_success: true,
            code_status: StatusCode::OK.as_u16(),
            message: String::from("Login succcessfully!"),
            role: Some(role),
            token: Some(token)
        }).unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorMessage {
    pub code_status: u16,
    pub message: String,
}

impl ErrorMessage {
    pub fn new(code_status: StatusCode, message: String) -> ErrorMessage {
        ErrorMessage {
            code_status: code_status.as_u16(),
            message
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: RecordId,
    pub role: Role,
    pub exp: usize,
}



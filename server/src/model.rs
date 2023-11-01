use self::database_model::Role;
use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub mod database_model;

pub const SECRECT_KEY: &str = "FER201m";
pub const SUPABASE_URL: &str = "https://ejcclhohxygecsqgclkj.supabase.co/rest/v1";
pub const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVqY2NsaG9oeHlnZWNzcWdjbGtqIiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTgxOTU0NDEsImV4cCI6MjAxMzc3MTQ0MX0.-hlpzR3idmtQtp3tXHDnIHGj3n9IJNXHyEtKAXlOf7s";

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
            head.append(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            head
        });
        GeneralResponse {
            status_code,
            header,
            body,
        }
    }
    pub fn unauthorized(message: Option<String>) -> GeneralResponse {
        let mut head = HeaderMap::new();
        head.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        GeneralResponse {
            status_code: StatusCode::UNAUTHORIZED,
            header: head,
            body: BodyMessage {
                code_status: StatusCode::UNAUTHORIZED.as_u16(),
                message: message.unwrap_or(String::from("Unauthorized!")),
            }
            .to_json(),
        }
    }
    pub fn internal_server_error(message: Option<String>) -> GeneralResponse {
        let mut head = HeaderMap::new();
        head.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        GeneralResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            header: head,
            body: BodyMessage {
                code_status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: message.unwrap_or(String::from("Internal server error!")),
            }
            .to_json(),
        }
    }
    pub fn not_found(message: Option<String>) -> GeneralResponse {
        let mut head = HeaderMap::new();
        head.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        GeneralResponse {
            status_code: StatusCode::NOT_FOUND,
            header: head,
            body: BodyMessage {
                code_status: StatusCode::NOT_FOUND.as_u16(),
                message: message.unwrap_or(String::from("Not found!")),
            }
            .to_json(),
        }
    }
    pub fn ok(body: String) -> GeneralResponse {
        let mut head = HeaderMap::new();
        head.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        GeneralResponse {
            status_code: StatusCode::OK,
            header: head,
            body,
        }
    }
    pub fn bad_request(message: String) -> GeneralResponse {

        let mut head = HeaderMap::new();
        head.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        GeneralResponse {
            status_code: StatusCode::BAD_REQUEST,
            header: head,
            body: BodyMessage {
                code_status: StatusCode::BAD_REQUEST.as_u16(),
                message
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
    pub code_status: u16,
    pub message: String,
    pub token: Option<String>,
    pub role: Option<Role>,
    pub user_id: Option<String>,
    pub full_name: Option<String>
}

impl LoginSuccess {
    pub fn to_json(token: String, role: Option<Role>, user_id: Option<String>, full_name: Option<String>) -> String {
        serde_json::to_string(&LoginSuccess {
            code_status: StatusCode::OK.as_u16(),
            message: String::from("Login successfully!"),
            token: Some(token),
            role,
            user_id,
            full_name
        })
        .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyMessage {
    pub code_status: u16,
    pub message: String,
}

impl BodyMessage {
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
    pub user_id: String,
    pub role: Role,
    pub exp: usize,
}

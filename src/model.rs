use axum::{
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use self::database_model::RoleName;

pub mod database_model;

pub const SECRECT_KEY: &str = "FER201m";


#[derive(Debug, Clone)]
pub struct GeneralResponse {
    pub status_code: StatusCode,
    pub header: HeaderMap,
    pub body: String,
}

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
}

impl IntoResponse for GeneralResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.header, self.body).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginStatus {
    pub login_success: bool,
    pub code_status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>
}

impl LoginStatus {
    pub fn login_success(token: String) -> String{
        serde_json::to_string(&LoginStatus {
            login_success: true,
            code_status: StatusCode::OK.as_u16(),
            message: String::from("Login succcessfully!"),
            token: Some(token)
        }).unwrap()
    }
    pub fn login_failed() -> String{
        serde_json::to_string(&LoginStatus {
            login_success: false,
            code_status: StatusCode::UNAUTHORIZED.as_u16(),
            message: String::from("Login failed!"),
            token: None
        }).unwrap()
    }
    pub fn internal_server_error() -> String{
        serde_json::to_string(&LoginStatus {
            login_success: false,
            code_status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("Internal server error!"),
            token: None
        }).unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: RecordId,
    pub role: RoleName,
    pub exp: usize,
}



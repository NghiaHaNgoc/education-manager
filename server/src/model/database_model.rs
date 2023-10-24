use serde::{Deserialize, Serialize};
use std::fmt;
use surrealdb::opt::RecordId;
use serde_with::skip_serializing_none;

/* NOTE: convention
*  Record link data struct will prefix in name with "Rcl"
*  Normal record will prefix with "Nm"
*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Role {
    STUDENT,
    TEACHER,
    ADMIN,
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::STUDENT => write!(f, "STUDENT"),
            Role::TEACHER => write!(f, "TEACHER"),
            Role::ADMIN => write!(f, "ADMIN")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gender {
    MALE,
    FEMALE
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::MALE => write!(f, "MALE"),
            Gender::FEMALE => write!(f, "FEMALE")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Option<RecordId>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
    pub user_profile: Option<RecordId>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralProfile {
    pub id: Option<RecordId>,
    pub address: Option<String>,
    pub birth: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub fullname: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminProfile {
    pub id: Option<RecordId>,
    pub address: Option<String>,
    pub birth: Option<String>,
    pub email: Option<String>,
    pub fullname: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentProfile {
    pub id: Option<RecordId>,
    pub fullname: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeacherProfile {
    pub id: Option<RecordId>,
    pub fullname: Option<String>,
}

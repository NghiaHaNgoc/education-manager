use serde::{Deserialize, Serialize};
use std::fmt;
use serde_with::skip_serializing_none;

/* NOTE: convention
*  Record link data struct will prefix in name with "Rcl"
*  Normal record will prefix with "Nm"
*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Role {
    Student,
    Lecturer,
    Admin,
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Student => write!(f, "Student"),
            Role::Lecturer => write!(f, "Lecturer"),
            Role::Admin => write!(f, "Admin")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "MALE"),
            Gender::Female => write!(f, "FEMALE")
        }
    }
}


#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Admin {
    #[serde(skip_serializing)]
    pub id: Option<u32>,
    pub admin_id: Option<String>,
    pub full_name: Option<String>,
    pub birth: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lecturer {
    #[serde(skip_serializing)]
    pub id: Option<u32>,
    pub lecturer_id: Option<String>,
    pub full_name: Option<String>,
    pub birth: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
    #[serde(skip_serializing)]
    pub id: Option<u32>,
    pub student_id: Option<String>,
    pub full_name: Option<String>,
    pub birth: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>
}


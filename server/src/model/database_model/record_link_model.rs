use serde::{Serialize, Deserialize};
use surrealdb::opt::RecordId;

use super::Role;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RclAccount {
    pub id: Option<RecordId>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
    pub user_profile: Option<RecordId>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RclUserProfile {
    pub id: Option<RecordId>,
    pub fullname: Option<String>,
}


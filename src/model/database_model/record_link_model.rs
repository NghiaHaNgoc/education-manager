use serde::{Serialize, Deserialize};
use surrealdb::opt::RecordId;

use super::RoleName;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RclAccount {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<RclRoles>,
    pub user_profile: Option<RclUserProfile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RclRoles {
    pub id: Option<RecordId>,
    pub role: RoleName
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RclUserProfile {
    pub id: Option<RecordId>,
    pub fullname: Option<String>,
}


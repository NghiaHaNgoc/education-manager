use serde::{Serialize, Deserialize};
use surrealdb::opt::RecordId;

use super::RoleName;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NmAccount {
    username: Option<String>,
    password: Option<String>,
    role: Option<RecordId>,
    user_profile: Option<RecordId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NmRoles {
    id: Option<RecordId>,
    role: Option<RoleName>
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    id: Option<RecordId>,
    fullname: Option<String>,
}

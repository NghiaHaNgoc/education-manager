use serde::{Serialize, Deserialize};

/* NOTE: convention
*  Record link data struct will prefix in name with "Rcl"
*  Normal record will prefix with "Nm"
*/
pub mod record_link_model;
pub mod normal_model;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RoleName {
    STUDENT,
    TEACHER,
    ADMIN,
}


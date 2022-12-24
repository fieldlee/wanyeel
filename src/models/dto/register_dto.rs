use serde::{Deserialize, Serialize};
use crate::models::{RegisterType,UserType};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RegisterDTO {
    account: Option<String>,
    pwd: Option<String>,
    phone: Option<String>,
    code: Option<String>,
    register_type: Option<RegisterType>,
    user_type: Option<UserType>,
    join_code: Option<String>,
}

use serde::{Deserialize, Serialize};
use crate::models::{RegisterType,UserType};
use validator_derive::Validate;

#[derive(Clone, Debug, Serialize, Validate,Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RegisterDTO {
    #[validate(required)]
    #[validate(length(min = 5, message = "账号最少5个字符"))]
    account: Option<String>,
    pwd: Option<String>,
    #[validate(length(min = 11, message = "账号最少11个字符"))]
    phone: Option<String>,
    code: Option<String>,
    #[validate(required)]
    register_type: Option<RegisterType>,
    user_type: Option<UserType>,
    join_code: Option<String>,
}

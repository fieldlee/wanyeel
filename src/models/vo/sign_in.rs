
use crate::models::dto::{user_dto::UserDTO, admin_dto::AdminDTO};
use serde::{Deserialize, Serialize};

/**
*struct:AdminSignInVO
*desc:登录数据
*author:String
*email:249608904@qq.com
*/
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AdminSignInVO {
    user: Option<AdminDTO>,
    access_token: String,
}

impl ToString for AdminSignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

/**
*struct:SignInVO
*desc:登录数据
*author:String
*email:249608904@qq.com
*/
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInVO {
    user: Option<UserDTO>,
    access_token: String,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ApiSignInVO {
    user: Option<UserDTO>,
    access_token: String,
}

impl ToString for ApiSignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}


use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct WxSignInVo {
    code: Option<String>,
    login_type: Option<String>,
    iv: Option<String>,
    encrypted_data: Option<String>,
    signature: Option<String>,
    raw_data: Option<String>,
}

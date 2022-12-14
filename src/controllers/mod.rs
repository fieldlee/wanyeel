use axum::{routing::get,routing::post, Router};
pub mod admin_controller;
pub mod user_controller;
pub mod report_controller;
pub mod wx_controller;
use user_controller::*;
use admin_controller::*;
use report_controller::*;

pub fn init_need_admin_router() -> Router {
    Router::new()
    .route("/service", post(save_service_info))
    .route("/update_service", post(update_service_info))
    .route("/service", get(get_service_info))
}

pub fn init_need_auth_router() -> Router {
    Router::new()
    .route("/user/:id", get(get_user_info))
    .route("/update_user", post(update_user_info))
}

pub fn init_noneed_auth_router() -> Router {
    Router::new()
    //-------------------------------------登录服务-------------------------------------------------------
    .route("/captcha/:uuid", get(captcha_base64))

    .route("/captcha/png/:uuid", get(captcha_png))
    
    .route("/login", post(user_login))

    .route("/admin_login", post(admin_login))

    .route("/register", post(user_register))

    .route("/check_keys", post(check_user_account))

    .route("/send_sms", post(send_phone_sms))
    
    .route("/login_phone", post(user_login_phone))
    // save 意见反馈信息
    .route("/save_report", post(report))
}
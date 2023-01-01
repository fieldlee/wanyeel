use axum::response::IntoResponse;
use crate::models::dto::admin_dto::ServiceDTO;
use crate::models::request::ServiceQuery;
use crate::services::admin_service::{AdminAuthService, ServiceInfoService};
use crate::utils::RespVO;
use crate::cache::cache::CacheService;
use axum::Json;
use validator::Validate;
use crate::utils::error::Error;
use crate::models::dto::sign_in::AdminSignInDTO;
use crate::APPLICATION_CONTEXT;

//用户登录
pub async fn admin_login(Json(sign): Json<AdminSignInDTO>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    let admin_auth_service = APPLICATION_CONTEXT.get::<AdminAuthService>();
    if let Err(e) = sign.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    println!("========");
    
    let vo = admin_auth_service.sign_in(&sign).await;
    return RespVO::from_result(&vo).resp_json();
}

// 更新服务配置
pub async fn save_service_info(Json(sign): Json<ServiceDTO>) -> impl IntoResponse {
    let service_info_service = APPLICATION_CONTEXT.get::<ServiceInfoService>();
    let vo = service_info_service.save_service_info(sign).await;
    return RespVO::from_result(&vo).resp_json();
}

// 更新服务配置
pub async fn update_service_info(Json(sign): Json<ServiceDTO>) -> impl IntoResponse {
    let service_info_service = APPLICATION_CONTEXT.get::<ServiceInfoService>();
    let vo = service_info_service.update_service_info(sign).await;
    return RespVO::from_result(&vo).resp_json();
}

// 查询服务配置
pub async fn get_service_info() -> impl IntoResponse {
    let service_info_service = APPLICATION_CONTEXT.get::<ServiceInfoService>();
    let service_query = ServiceQuery::default();

    let query_result = service_info_service.get_service_info(&service_query).await;

    return RespVO::from_result(&query_result).resp_json();
}
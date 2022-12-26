use crate::services::report_service::ReportService;
use crate::models::dto::report_dto::ReportDTO;
use axum::{response::IntoResponse};
use crate::APPLICATION_CONTEXT;
use axum::Json;
use validator::Validate;
use crate::utils::RespVO;
use crate::utils::error::Error;
// 意见反馈
pub async fn report(Json(report): Json<ReportDTO>) -> impl IntoResponse {
    let report_service = APPLICATION_CONTEXT.get::<ReportService>();

    if let Err(e) = report.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }

    let vo = report_service.save_report(report).await;

    return RespVO::from_result(&vo).resp_json();
}
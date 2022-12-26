use serde::{Deserialize, Serialize};
use crate::models::{ReportType};
use crate::models::entitys::report_entity::Report;
use validator_derive::Validate;

#[derive(Clone, Debug, Serialize, Validate,Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ReportDTO {
    id: Option<i64>,
    #[validate(length(min = 4, message = "账号最少4个字符"))]
    name: Option<String>,
    report_type: Option<ReportType>,
    #[validate(length(min = 11, message = "账号最少11个字符"))]
    phone: Option<String>,
    email: Option<String>,
    #[validate(length(min = 10, message = "账号最少10个字符"))]
    content: Option<String>,
}


impl Into<Report> for ReportDTO {
    fn into(self) -> Report {
        Report {
            id: self.id().clone(),
            report_type: self.report_type().clone(),
            name: self.name().clone(),
            content: self.content().clone(),
            phone: self.phone().clone(),
            email: self.email().clone(),
        }
    }
}

impl From<Report> for ReportDTO {
    fn from(arg: Report) -> Self {
        Self {
            id: arg.id,
            phone: arg.phone,
            email: arg.email,
            report_type: arg.report_type,
            content: arg.content,
            name: arg.name,
        }
    }
}
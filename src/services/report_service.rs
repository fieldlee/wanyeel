use crate::models::{
    dto::report_dto::ReportDTO,
    entitys::report_entity::Report,
    request::UserQuery,
};
use crate::utils::error::Result;
use rbatis::rbatis::Rbatis;
use crate::{crud::crud_service::CrudService, APPLICATION_CONTEXT};

pub struct ReportService;
impl Default for ReportService {
    fn default() -> Self {
        ReportService {
        }
    }
}
impl ReportService {
    //根据id查询用户
    pub async fn save_report(&self, arg: ReportDTO) -> Result<i64> {
        let mut entity: Report = arg.into();
        let id = self.save(&mut entity).await;
        return Ok(id.unwrap());
    }
}

impl CrudService<Report, ReportDTO,UserQuery> for ReportService {
    fn get_wrapper(arg:&UserQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(
        &self,
        common: crate::models::entitys::CommonField,
        data: &mut Report,
    ) {

    }
}

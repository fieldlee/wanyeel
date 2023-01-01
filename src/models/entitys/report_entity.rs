use crate::models::{ReportType};

#[crud_table(table_name:report)]
#[derive(Clone, Debug)]
pub struct Report {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub report_type: Option<ReportType>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub content: Option<String>,
}

impl_field_name_method!(Report {
    id,
    name,
    report_type,
    phone,
    email,
    content,
});

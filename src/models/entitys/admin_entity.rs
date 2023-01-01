#[crud_table(table_name:admin_info)]
#[derive(Clone, Debug)]
pub struct Admin {
    pub id: Option<i64>,
    pub account: Option<String>,
    pub pwd: Option<String>,
    pub status:Option<i8>,
}

impl_field_name_method!(Admin {
    id,
    account,
    pwd,
    status,
});


#[crud_table(table_name:service_info)]
#[derive(Clone, Debug)]
pub struct Service {
    pub id: Option<i64>,
    pub service_id: Option<String>,
    pub service_name: Option<String>,
    pub service_desc: Option<String>,
    pub service_port: Option<String>,
    pub service_status:Option<i8>,
    pub service_num:Option<i8>,
}

impl_field_name_method!(Service {
    id,
    service_id,
    service_name,
    service_desc,
    service_port,
    service_status,
    service_num,
});

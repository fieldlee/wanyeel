use serde::{Deserialize, Serialize};
use crate::models::entitys::admin_entity::{Admin, Service};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AdminDTO {
    id: Option<i64>,
    account: Option<String>,
    pwd: Option<String>,
    status: Option<i8>,
}

impl Into<Admin> for AdminDTO {
    fn into(self) -> Admin {
        Admin {
            id: self.id().clone(),
            account: self.account().clone(),
            pwd: self.pwd().clone(),
            status: self.status().clone(),
        }
    }
}

impl From<Admin> for AdminDTO {
    fn from(arg: Admin) -> Self {
        Self {
            id: arg.id,
            account: arg.account,
            pwd: arg.pwd,
            status: arg.status,
        }
    }
}




#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ServiceDTO {
    id: Option<i64>,
    service_id: Option<String>,
    service_name: Option<String>,
    service_desc: Option<String>,
    service_port: Option<String>,
    service_num:Option<i8>,
    service_status:Option<i8>,  // 0 弃用  1 正常
}

impl Into<Service> for ServiceDTO {
    fn into(self) -> Service {
        Service {
            id: self.id().clone(),
            service_id: self.service_id().clone(),
            service_name: self.service_name().clone(),
            service_desc: self.service_desc().clone(),
            service_port: self.service_port().clone(),
            service_num:self.service_num().clone(),
            service_status:self.service_status().clone(),
        }
    }
}

impl From<Service> for ServiceDTO {
    fn from(arg: Service) -> Self {
        Self {
            id: arg.id,
            service_id: arg.service_id,
            service_name: arg.service_name,
            service_desc: arg.service_desc,
            service_port: arg.service_port,
            service_num: arg.service_num,
            service_status: arg.service_status,
        }
    }
}
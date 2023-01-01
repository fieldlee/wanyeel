use crate::models::UserType;
use crate::models::dto::admin_dto::ServiceDTO;
use crate::models::entitys::admin_entity::Service;
use crate::models::vo::sign_in::AdminSignInVO;
use crate::models::{
    entitys::admin_entity::Admin,
};
use rbatis::rbatis::Rbatis;
use crate::utils::error::Error;
use crate::utils::error::Result;
use crate::utils::password_encoder::PasswordEncoder;
use crate::config::config::ApplicationConfig;
use crate::models::dto::sign_in::{AdminSignInDTO};
use crate::models::request::ServiceQuery;
use crate::models::vo::jwt::JWTToken;
use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use crate::{APPLICATION_CONTEXT,crud::crud_service::CrudService};

/**
*struct:AdminAuthService
*desc:用户权限服务  登录 错误重试
*author:String
*email:249608904@qq.com
*/
pub struct AdminAuthService {}

impl Default for AdminAuthService {
    fn default() -> Self {
        AdminAuthService {}
    }
}

impl AdminAuthService {
    pub async fn sign_in(&self, arg: &AdminSignInDTO) -> Result<AdminSignInVO> {
        /*验证码 验证*/
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let user: Option<Admin> = rb
            .fetch_by_wrapper(rb.new_wrapper().eq(Admin::account(), &arg.username()))
            .await?;
        
        println!("get user info :{:?}", user);

        let user = user.ok_or_else(|| {
            Error::from(format!(
                "账号:{} 不存在!",
                arg.username().clone().unwrap_or_default()
            ))
        })?;
        println!("get user info 2 :{:?}", user);
        if !user.status.eq(&Some(1)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        if !PasswordEncoder::verify(
            user.pwd
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            arg.password()
                .as_ref()
                .ok_or_else(|| Error::from("密码不能为空"))?,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        if error.is_some() {
            return Err(error.unwrap());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        return Ok(sign_in_vo);
    }

    /**
     *method:get_admin_info
     *desc:获取用户信息
     *author:String
     *email:249608904@qq.com
     */
    pub async fn get_user_info(&self, user: &Admin) -> Result<AdminSignInVO> {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        //去除密码，增加安全性
        let mut user = user.clone();
        user.pwd = None;
        // let agency_code = user.agency_code.clone();
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let mut sign_vo = AdminSignInVO::default();
        sign_vo.set_user(Some(user.clone().into()));
        //提前查找所有权限，避免在各个函数方法中重复查找
        let mut jwt_token = JWTToken::default();
        jwt_token.set_id(user_id);
        jwt_token.set_user_type(Some(UserType::Admin));
        jwt_token.set_from(Default::default());
        jwt_token.set_username(user.account.clone().unwrap_or_default());
        jwt_token.set_exp(DateTimeNative::now().timestamp_millis() as usize);
        sign_vo.set_access_token(jwt_token.create_token(cassie_config.jwt_secret())?);
        return Ok(sign_vo);
    }
}




/**
*struct:ServiceInfoService
*desc: 服务配置
*author:String
*email:249608904@qq.com
*/
pub struct ServiceInfoService {}

impl Default for ServiceInfoService {
    fn default() -> Self {
        ServiceInfoService {}
    }
}

impl ServiceInfoService {
    /**
     *method:save_service_info
     *desc:保存服务配置信息
     *author:String
     *email:249608904@qq.com
     */
    pub async fn save_service_info(&self, arg: ServiceDTO) -> Result<i64> {
        let mut entity = arg.into();
        let id = self.save(&mut entity).await;
        return Ok(id.unwrap());
    }

    /**
     *method:get_service_info
     *desc:获得服务配置信息
     *author:String
     *email:249608904@qq.com
     */
    pub async fn get_service_info(&self,query:&ServiceQuery) -> Result<Vec<ServiceDTO>> {
        return self.list(query).await;
    }

    /**
     *method:update_service_info
     *desc:获得服务配置信息
     *author:String
     *email:249608904@qq.com
     */
    pub async fn update_service_info(&self, arg: ServiceDTO) -> Result<i64> {
        let entity: Service = arg.into();
        /*保存到数据库*/
        if let Some(id) = entity.id {
            let user = self.get(id.to_string()).await;
            match user {
                Ok(_) => {
                    self.update_by_id(id.to_string(), &entity).await;
                    return Ok(id);
                },
                Err(err) => {
                    return Ok(0);
                }
            }
        } 
        return Ok(0);
    }
}


impl CrudService<Service, ServiceDTO,ServiceQuery> for ServiceInfoService {
    fn get_wrapper(arg:&ServiceQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(
        &self,
        common: crate::models::entitys::CommonField,
        data: &mut Service,
    ) {

    }
}

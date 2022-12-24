use crate::config::config::ApplicationConfig;
use crate::models::{
    dto::user_dto::{UserDTO, WechatUserDTO},
    entitys::user_entity::{User, WechatUser},
    vo::wx::WxSignInVo,
};
use wx::app::{auth::get_session_key, resolve_data, WxappSessionKey};

use crate::{
    middleware::get_local,
    services::user_service::UserService,
    services::wechat_user_service::WechatUserService,
    crud::crud_service::CrudService,
    APPLICATION_CONTEXT,
};
use crate::utils::error::Error;
use crate::utils::error::Result;
use rbatis::{crud::CRUD, rbatis::Rbatis};
use crate::cache::cache::CacheService;
//新增或更新用户
pub async fn save_or_update_user(user: WechatUserDTO) -> i64 {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let wechat_user_service = APPLICATION_CONTEXT.get::<WechatUserService>();
    //判断unionid  存在根据unionid判断
    let mut uid = match user.unionid() {
        Some(unionid) => {
            let list = wechat_user_service
                .fetch_list_by_column(WechatUser::unionid(), &vec![unionid.clone()])
                .await
                .unwrap();
            if list.len() > 0 {
                let mut userinfo = list.get(0).unwrap().clone();
                let uid = userinfo.id().clone().unwrap();
                userinfo.set_nickname(user.nickname().clone());
                userinfo.set_headimgurl(user.headimgurl().clone());
                //执行更新逻辑
                wechat_user_service
                    .update_by_id(uid.to_string(), &mut userinfo.into())
                    .await;
                uid
            } else {
                0
            }
        }
        None => 0,
    };
    //如果unionid不存在 则根据openid判断
    if uid == 0 {
        uid = match user.routine_openid() {
            Some(routine_openid) => {
                let list = wechat_user_service
                    .fetch_list_by_column(
                        WechatUser::routine_openid(),
                        &vec![routine_openid.clone()],
                    )
                    .await
                    .unwrap();
                if list.len() > 0 {
                    let userinfo = list.get(0).unwrap();
                    let uid = userinfo.id().clone().unwrap();
                    //执行更新逻辑
                    wechat_user_service
                        .update_by_id(uid.to_string(), &mut user.clone().into())
                        .await;
                    return uid;
                }
                0
            }
            None => 0,
        };
        if uid == 0 {
            //执行新增逻辑
            uid = insert_user(user).await;
        }
    }
    uid
}

fn build_user_info(user: &WechatUserDTO) -> UserDTO {
    let mut user_info = UserDTO::default();
    user_info.set_avatar(user.headimgurl().clone());
    user_info.set_nickname(user.nickname().clone());
    user_info
}

pub async fn insert_user(mut user: WechatUserDTO) -> i64 {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    let user_dto = build_user_info(&user);
    //插入用户
    let result = rb.save::<User>(&mut user_dto.into(), &[]).await;
    match result {
        Ok(d) => {
            user.set_id(d.last_insert_id.clone());
            //插入微信用户
            rb.save::<WechatUser>(&mut user.into(), &[]).await;
            d.last_insert_id.unwrap()
        }
        Err(_) => 0,
    }
}

pub async fn wxapp_auth(sign: WxSignInVo) -> Result<i64> {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    //获取 session_key 如果已经授权了  直接拿到session_key

    //如果授权code 和session_key 都不存在 则参数异常
    if sign.code().is_none() {
        return Err(Error::E("授权失败,参数有误!".to_string()));
    }
    let mut session_key = "".to_string();
    let mut unionid = "".to_string();
    let mut openid = "".to_string();
    let mut wechat_user = WechatUserDTO::default();
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //新登录用户
    //如果code存在 session_key不存在 则需要根据code拿到session_key
    if sign.code().is_some() && session_key.is_empty() {
        match get_session_key(
            config.wxapp().appid(),
            config.wxapp().secret(),
            &sign.code().clone().unwrap(),
        )
        .await
        {
            Ok(data) => {
                println!("session:{:?}", data);
                let data: WxappSessionKey = serde_json::from_value(data).unwrap();
                session_key = data.session_key.clone();
                openid = data.openid.clone();
                unionid = if data.unionid.is_some() {
                    data.unionid.clone().unwrap()
                } else {
                    "".to_string()
                };
            }
            Err(e) => {
                return Err(Error::E(
                    "获取session_key失败，请检查您的配置！".to_string(),
                ));
            }
        };
    }
    wechat_user.set_session_key(Some(session_key.clone()));
    //解密获取 用户信息 组装数据
    match resolve_data(
        session_key.clone(),
        sign.iv().clone().unwrap(),
        sign.encrypted_data().clone().unwrap(),
    ) {
        Ok(wx_info) => {
            wechat_user.set_nickname(wx_info.nick_name); //昵称
            wechat_user.set_headimgurl(wx_info.avatar_url); //头像
            wechat_user.set_sex(wx_info.gender); //性别
            wechat_user.set_city(wx_info.city); //市
            wechat_user.set_country(wx_info.country);
            wechat_user.set_province(wx_info.province); //省
            wechat_user.set_language(wx_info.language); //语言
        }
        Err(e) => {
            return Err(Error::E(format!("获取会话密匙失败{}", e.to_string())));
        }
    }
    //openid为空则授权异常
    if openid.is_empty() {
        return Err(Error::E("openid获取失败".to_string()));
    }
    wechat_user.set_routine_openid(Some(openid)); //设置openid
    wechat_user.set_unionid(Some(unionid));
    //新增或更新用户
    return Ok(save_or_update_user(wechat_user).await);
}

pub async fn binding_phone(sign: WxSignInVo) -> Result<String> {
    let request_model = get_local().unwrap();
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    match get_session_key(
        config.wxapp().appid(),
        config.wxapp().secret(),
        &sign.code().clone().unwrap(),
    )
    .await
    {
        Ok(data) => {
            let data: WxappSessionKey = serde_json::from_value(data).unwrap();
            //解析用户信息
            match resolve_data(
                data.session_key,
                sign.iv().clone().unwrap(),
                sign.encrypted_data().clone().unwrap(),
            ) {
                Ok(wx_info) => {
                    if wx_info.pure_phone_number.is_none()
                        || wx_info.pure_phone_number.clone().unwrap().is_empty()
                    {
                        return Err(Error::E("手机号获取失败".to_string()));
                    }
                    //执行更新逻辑
                    let user_service = APPLICATION_CONTEXT.get::<UserService>();
                    let mut user = user_service
                        .get(request_model.uid().to_string())
                        .await
                        .unwrap();
                    user.set_phone(wx_info.pure_phone_number.clone());
                    user_service
                        .update_by_id(request_model.uid().to_string(), &mut user.into())
                        .await;
                    Ok(wx_info.pure_phone_number.clone().unwrap())
                }
                Err(e) => {
                    return Err(Error::E(format!("获取会话密匙失败{}", e.to_string())));
                }
            }
        }
        Err(e) => {
            return Err(Error::E(
                "获取session_key失败，请检查您的配置！".to_string(),
            ));
        }
    }
}

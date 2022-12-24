use crate::{ APPLICATION_CONTEXT};
use crate::config::config::ApplicationConfig;
use crate::utils::error::Result;
use urlencoding::encode;
/**
*struct:SMSService
*desc: 发送短信
*author:String
*email:249608904@qq.com
*/
pub struct SmsSendService {}

impl Default for SmsSendService {
    fn default() -> Self {
        SmsSendService {}
    }
}

impl SmsSendService {
    pub async fn send_vcode(&self, phone: String,code:String) -> Result<()> {
        let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        let sms_url = config.sms_send_url();
        let sms_username = config.sms_send_username();
        let sms_password_md5 = config.sms_send_password_md5();
        let sms_send_apikey = config.sms_send_apikey();
        let sms_send_encode = config.sms_send_encode();

        let content = format!("【万业工具】您好，你的本次验证码是：{}。请五分钟内完成验证。如非本人操作请忽略本短信。",code);

        let content = encode(content.as_str());

        let full_url = format!("{}?username={}&password_md5={}&apikey={}&mobile={}&content={}&encode={}",
                                        sms_url,sms_username,sms_password_md5,sms_send_apikey,phone,content,sms_send_encode);

        println!("full_url:{}",full_url);

        let body = reqwest::get(full_url)
                                        .await?
                                        .text()
                                        .await?;

        if body.contains("success"){

        } else {
            println!("erorr:{}",body);
        }                          
        return Ok(());
    }
}
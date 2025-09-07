use crate::EmailConfig;
use crate::PulseError;
use crate::PulseResult;
use handlebars::Handlebars;
use include_dir::{Dir, include_dir};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::header,
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static VERIFICATION_CODES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static EMAIL_TEMPLATE: Dir = include_dir!("config");

pub async fn send_email(receiver_email: &str, email_config: &EmailConfig) -> PulseResult<()> {
    let template = EMAIL_TEMPLATE
        .get_file("emailTemplate.txt")
        .ok_or_else(|| PulseError::PulseStdError("emailTemplate.txt not found".to_string()))?;
    let content = template.contents_utf8().ok_or_else(|| {
        PulseError::PulseStdError("emailTemplate.txt contents not found".to_string())
    })?;

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("email_template", content)?;

    let render = handlebars.render("email_template", &generate_code()?)?;

    let message = Message::builder()
        .from(email_config.username.parse()?) // 设置发件人
        .reply_to(email_config.username.parse()?) // 设置收件人回复的地址
        .to(receiver_email.parse()?) // 设置收件人
        .subject("【安全提醒】您的密码重置验证码") // 设置邮件主题
        .header(header::ContentType::TEXT_PLAIN) // 设置邮件内容类型为文本
        .body(render)?; // 设置邮件内容

    // 配置SMTP认证凭据
    let credentials =
        Credentials::new(email_config.username.clone(), email_config.password.clone());

    // 创建异步SMTP传输器
    let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&email_config.host)?
        .port(email_config.port) // QQ邮箱SMTP TLS端口
        .credentials(credentials)
        // 使用Required而不是Wrapper，避免TLS配置冲突
        .tls(Tls::Required(
            TlsParameters::builder(email_config.host.clone())
                .dangerous_accept_invalid_certs(false) // 开发环境可以接受无效证书
                .build()?,
        ))
        .timeout(Some(std::time::Duration::from_secs(email_config.timeout))) // 设置超时时间
        .build();

    // 异步发送邮件
    smtp_transport.send(message).await?;
    Ok(())
}

// 随机生成六位数的验证码
fn generate_code() -> PulseResult<HashMap<String, String>> {
    // 获取当前时间的纳秒数
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();

    // 使用纳秒数生成6位验证码
    let code = format!("{}", (nanos % 900000) + 100000); // 确保是6位数

    // 将验证码添加到静态变量中
    let mut codes = VERIFICATION_CODES.lock().unwrap();
    codes.push(code.clone());

    let mut result = HashMap::new();
    result.insert("CODE".to_string(), code);
    Ok(result)
}

// 验证用户输入的验证码是否正确，并在验证成功后从列表中删除
pub fn verify_code(input_code: &str) -> bool {
    let mut codes = VERIFICATION_CODES.lock().unwrap();

    // 查找验证码在列表中的位置
    if let Some(pos) = codes.iter().position(|code| code == input_code) {
        // 验证成功，从列表中删除该验证码
        codes.remove(pos);
        return true;
    }

    false
}

#[cfg(test)]
mod email_test {
    use crate::{
        EmailConfig,
        utils::email_util::{self},
    };

    #[tokio::test]
    async fn test() {
        let config = EmailConfig {
            host: "smtp.qq.com".to_string(),
            port: 587,
            username: "1220531071@qq.com".to_string(),
            password: "ulfymcwntccvhabd".to_string(),
            timeout: 10,
        };

        match email_util::send_email("1220531071@qq.com", &config).await {
            Ok(_) => {
                println!("邮件发送成功");
            }
            Err(e) => {
                println!("邮件发送失败: {:?}", e);
            }
        }
    }
}

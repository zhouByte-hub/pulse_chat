use crate::utils::token::PulseClaims;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use std::future;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

// TokenFilter中间件结构体
pub struct TokenFilter<S> {
    service: Arc<S>,
    public_routes: Arc<Vec<String>>,
    secret: Arc<String>,
}

impl<S> TokenFilter<S> {
    /// 创建新的TokenFilter实例
    pub fn new(service: S, public_routes: Vec<String>, secret: String) -> Self {
        Self {
            // 使用Arc包装服务，实现线程安全的共享所有权
            service: Arc::new(service),
            // 使用Arc包装公共路由列表，实现线程安全的共享所有权
            public_routes: Arc::new(public_routes),
            // 使用Arc包装密钥，实现线程安全的共享所有权
            secret: Arc::new(secret),
        }
    }
}

// 实现Service trait
impl<S, B> Service<ServiceRequest> for TokenFilter<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // 委托给被包装的服务，检查其是否准备就绪
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 获取请求路径并转换为字符串
        let path = req.path().to_string();
        // 克隆公共路由列表，用于在异步块中使用
        let public_routes = self.public_routes.clone();
        // 克隆服务引用，用于在异步块中调用
        let service = self.service.clone();

        // 将异步块包装为Pin<Box<dyn Future>>
        let secret = self.secret.clone();
        Box::pin(async move {
            // 检查当前路径是否在白名单（公共路由）中
            if public_routes.contains(&path) {
                // 如果是公共路由，直接放行，调用后续服务处理请求
                return service.call(req).await;
            }

            // 从HTTP请求头中获取Authorization字段
            let auth_header = req.headers().get("Authorization");
            // 使用match表达式处理Authorization头
            let token = match auth_header {
                Some(header) => {
                    // 将HeaderValue转换为字符串切片，失败则返回未授权错误
                    let header_str = header.to_str().map_err(|_| {
                        actix_web::error::ErrorUnauthorized("请求不合法，缺少Token")
                    })?;

                    // 检查token格式是否为Bearer开头
                    if !header_str.starts_with("Bearer ") {
                        return Err(actix_web::error::ErrorUnauthorized("请求不合法，缺少Token"));
                    }

                    // 去掉"Bearer "前缀，提取纯token字符串
                    header_str[7..].to_string()
                }
                None => {
                    // 如果没有Authorization头，返回未授权错误
                    return Err(actix_web::error::ErrorUnauthorized("请求不合法，缺少Token"));
                }
            };

            // 定义JWT密钥（实际项目中应该从配置文件中读取）
            // let secret = "your_secret_key_here";
            // 从密钥字符串创建解码密钥
            // let decoding_key = DecodingKey::from_secret(secret.as_ref());
            // // 创建验证规则，使用HS256算法
            // let validation = Validation::new(Algorithm::HS256);

            // 尝试解码JWT token
            let user_id = PulseClaims::verify_token(&token, &secret)?;
            req.extensions_mut().insert(user_id);
            service.call(req).await
            // let a = match decode::<PulseClaims>(&token, &decoding_key, &validation) {
            //     Ok(token_data) => {
            //         // token验证成功，从声明中提取用户ID
            //         let user_id = token_data.claims.user_id;
            //         // 将用户ID存入请求扩展，供后续处理使用
            //         req.extensions_mut().insert(user_id);

            //         // 放行请求，调用后续服务处理
            //         service.call(req).await
            //     }
            //     Err(_) => {
            //         // token验证失败，返回未授权错误
            //         Err(actix_web::error::ErrorUnauthorized("Token不合法或已过期"))
            //     }
            // }
        })
    }
}

// TokenFilterFactory用于创建TokenFilter
pub struct TokenFilterFactory {
    public_routes: Vec<String>,
    token_secret: String,
}

impl TokenFilterFactory {
    /// 创建新的TokenFilterFactory实例
    pub fn new(public_routes: Vec<String>, token_secret: String) -> Self {
        Self {
            // 将公共路由列表存储在结构体中
            public_routes,
            token_secret,
        }
    }
}

// 实现Transform trait
impl<S, B> Transform<S, ServiceRequest> for TokenFilterFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TokenFilter<S>;
    type InitError = ();
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // 创建一个即时完成的Future，包含TokenFilter实例
        // 将服务和公共路由列表传递给TokenFilter构造函数
        future::ready(Ok(TokenFilter::new(
            service,
            self.public_routes.clone(),
            self.token_secret.clone(),
        )))
    }
}

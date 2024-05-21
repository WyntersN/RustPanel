use std::future::{ready, Ready};

use actix_session::{Session, SessionExt};
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};


pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
      


        let path = request.path();

        // 检查路由路径是否为登录路径
        if path.contains("/api/v1/login") || !path.contains("/api")  {
            // println!("Login");
            let res = self.service.call(request);
            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }



        // 检查请求的 Cookies 或 Headers 中是否有 token
        let token = request
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .map(|token| token.trim_start_matches("Bearer "));

        match token {
            Some(token) => {
                if let Ok(token_data) = decode::<String>(
                    &token,
                    &DecodingKey::from_secret("your_secret".as_ref()),
                    &Validation::default(),
                ) {
                    Session::insert(&request.get_session(), "6666", "9999").unwrap();
                    let res = self.service.call(request);

                    Box::pin(async move {
                        // forwarded responses map to "left" body
                        res.await.map(ServiceResponse::map_into_left_body)
                    })
                } else {
                    let (request, _pl) = request.into_parts();

                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({"code": 401, "message": "Unauthorized"}))
                        // constructed responses map to "right" body
                        .map_into_right_body();
                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                }
            }
            None => {
                let (request, _pl) = request.into_parts();

                let response = HttpResponse::Unauthorized()
                    .json(serde_json::json!({"code": 401, "message": "Unauthorized"}))
                    // constructed responses map to "right" body
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        }
    }
}

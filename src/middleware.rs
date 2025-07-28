use crate::AppState;
use actix_web::web;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::{
    rc::Rc,
    task::{Context, Poll},
};

pub struct ApiTokenMiddlewareFactory;
pub struct JwtMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for ApiTokenMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiTokenMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiTokenMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct ApiTokenMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ApiTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let state = req.app_data::<web::Data<AppState>>().unwrap().clone();

        // Allow /init without api token
        if req.path().ends_with("/init") {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        let token = req
            .headers()
            .get("x-api-token")
            .and_then(|v| v.to_str().ok());

        if let Some(token) = token {
            if state.api_tokens.lock().unwrap().contains(token) {
                let fut = self.service.call(req);
                return Box::pin(async move { fut.await });
            }
        }

        Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Invalid API token")) })
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // allow login and create_user without JWT
        if req.path().ends_with("/login") || req.path().ends_with("/create_user") {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());
        if let Some(hv) = auth_header {
            if hv.starts_with("Bearer ") {
                let token = &hv[7..];
                let state = req.app_data::<web::Data<AppState>>().unwrap();
                if crate::auth::validate_jwt(token, &state.jwt_secret) {
                    let fut = self.service.call(req);
                    return Box::pin(async move { fut.await });
                }
            }
        }
        Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Invalid JWT")) })
    }
}

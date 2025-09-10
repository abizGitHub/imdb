use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, http::header};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use std::pin::Pin;

struct AddHeader;

impl<S, B> Transform<S, ServiceRequest> for AddHeader
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AddHeaderMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AddHeaderMiddleware { service })
    }
}

struct AddHeaderMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AddHeaderMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(header::SERVER, header::HeaderValue::from_static("MyActixServer"));
            Ok(res)
        })
    }
}

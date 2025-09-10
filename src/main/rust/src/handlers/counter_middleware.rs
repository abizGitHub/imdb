use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, Responder,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use crate::handlers::db::CALL_COUNTER;
pub struct CallCounter {
    counter: Arc<Mutex<u32>>,
}

impl CallCounter {
    pub fn new() -> Self {
        CallCounter {
            counter: CALL_COUNTER.to_owned(),
        }
    }
}
impl<S, B> Transform<S, ServiceRequest> for CallCounter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CallCounterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CallCounterMiddleware {
            service,
            counter: self.counter.clone(),
        })
    }
}

pub struct CallCounterMiddleware<S> {
    service: S,
    counter: Arc<Mutex<u32>>,
}

impl<S, B> Service<ServiceRequest> for CallCounterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Increase the counter on each request
        {
            let mut num = self.counter.lock().unwrap();
            *num += 1;
            //println!("API call count: {}", *num);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

pub async fn get_counter() -> impl Responder {
    let c = *CALL_COUNTER.lock().unwrap();
    let msg = format!("call count:{c}");
    HttpResponse::Ok().body(msg)
}

use anyhow::Result;
use std::{future::Future, pin::Pin};

/// The handler function type
///
///
pub trait HandlerFn<R> {
    type Future;

    fn call(&self, request: R) -> Self::Future;
}

/// Impl HandlerFn for closures
///
///
impl<F, Req, Resp> HandlerFn<Req> for F
where
    F: Fn(Req) -> HandlerFuture<Resp>,
{
    type Future = HandlerFuture<Resp>;

    fn call(&self, request: Req) -> HandlerFuture<Resp> {
        self(request)
    }
}

pub type HandlerFuture<T> = Pin<Box<dyn Future<Output = Result<T>>>>;
pub type HandlerBox<Req, Resp> = Box<dyn HandlerFn<Req, Future = HandlerFuture<Resp>>>;

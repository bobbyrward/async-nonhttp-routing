use crate::{request::TestRequest, response::ResponseFuture};
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
impl<F> HandlerFn<TestRequest> for F
where
    F: Fn(TestRequest) -> ResponseFuture,
{
    type Future = ResponseFuture;

    fn call(&self, request: TestRequest) -> ResponseFuture {
        self(request)
    }
}

pub type HandlerFuture<T> = Pin<Box<dyn Future<Output = Result<T>>>>;

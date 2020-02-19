use crate::handler::{HandlerBox, HandlerFuture};
use crate::request::Request;
use crate::route::{Route, RouteBuilder};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::future::Future;

/// The request router
///
///
pub struct Router<Req, Resp> {
    routes: HashMap<u32, Vec<Route<Req, Resp>>>,
    default: Option<HandlerBox<Req, Resp>>,
}

impl<Req, Resp> Router<Req, Resp>
where
    Req: Request,
{
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            default: None,
        }
    }

    pub fn default<HFn, F>(&mut self, handler: HFn)
    where
        HFn: Fn(Req) -> F + 'static,
        F: Future<Output = Result<Resp>> + 'static,
    {
        self.default = Some(Box::new(move |req| -> HandlerFuture<Resp> {
            Box::pin(handler(req))
        }));
    }

    pub fn route(&mut self, code: u32) -> RouteBuilder<'_, Req, Resp> {
        RouteBuilder::new(self, code)
    }

    pub(crate) fn insert_route(&mut self, route: Route<Req, Resp>) {
        self.routes
            .entry(route.mti())
            .or_insert_with(Vec::new)
            .push(route);
        // self.routes.insert(route.mti, route);
    }

    pub fn request(&mut self, request: Req) -> Result<HandlerFuture<Resp>> {
        self.routes
            .get(&request.code())
            .and_then(|handlers| {
                for route in handlers {
                    if route.matches(&request) {
                        return Some(Ok(route.handler()));
                    }
                }
                None
            })
            .or_else(|| match self.default.as_ref() {
                Some(default) => Some(Ok(default)),
                None => Some(Err(anyhow!("No route"))),
            })
            .unwrap()
            .map(|route| route.call(request))
    }
}


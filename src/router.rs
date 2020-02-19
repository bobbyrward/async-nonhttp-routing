use crate::handler::HandlerFuture;
use crate::request::Request;
use crate::route::{Route, RouteBuilder};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// The request router
///
///
pub struct Router<Req, Resp> {
    routes: HashMap<u32, Vec<Route<Req, Resp>>>,
}

impl<Req, Resp> Router<Req, Resp>
where
    Req: Request,
{
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
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
        match self.routes.get(&request.code()) {
            Some(mti_handlers) => {
                for route in mti_handlers {
                    if route.matches(&request) {
                        return Ok(route.call(request));
                    }
                }

                Err(anyhow!("No route"))
            }
            None => Err(anyhow!("No route")),
        }
    }
}


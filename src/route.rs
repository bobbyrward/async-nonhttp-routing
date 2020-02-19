use crate::handler::{HandlerFn, HandlerFuture};
use crate::request::Request;
use crate::router::Router;
use std::collections::HashMap;

pub type HandlerBox<Req, Resp> = Box<dyn HandlerFn<Req, Future = HandlerFuture<Resp>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldValue {
    #[allow(dead_code)]
    Any,
    Exact(u32),
}

pub struct RouteBuilder<'a, Req, Resp> {
    mti: u32,
    fields: HashMap<u32, FieldValue>,
    router: &'a mut Router<Req, Resp>,
}

impl<'a, Req, Resp> RouteBuilder<'a, Req, Resp>
where
    Req: Request,
{
    pub(crate) fn new(router: &'a mut Router<Req, Resp>, mti: u32) -> Self {
        Self {
            mti,
            fields: HashMap::new(),
            router,
        }
    }

    pub fn field(mut self, index: u32, value: u32) -> Self {
        self.fields.insert(index, FieldValue::Exact(value));
        self
    }

    pub fn build<HFn>(self, handler: HFn)
    where
        HFn: HandlerFn<Req, Future = HandlerFuture<Resp>> + 'static,
    {
        self.router
            .insert_route(Route::new(self.mti, self.fields, Box::new(handler)));
    }
}

pub struct Route<Req, Resp> {
    mti: u32,
    fields: HashMap<u32, FieldValue>,
    handler: HandlerBox<Req, Resp>,
}

impl<Req, Resp> Route<Req, Resp>
where
    Req: Request,
{
    pub(crate) fn new(
        mti: u32,
        fields: HashMap<u32, FieldValue>,
        handler: HandlerBox<Req, Resp>,
    ) -> Self {
        Self {
            mti,
            fields,
            handler,
        }
    }

    pub fn mti(&self) -> u32 {
        self.mti
    }

    #[allow(dead_code)]
    pub fn expected_fields(&self) -> &HashMap<u32, FieldValue> {
        &self.fields
    }

    pub(crate) fn matches(&self, request: &Req) -> bool {
        if self.mti != request.code() {
            return false;
        }

        for (k, expected_value) in self.fields.iter() {
            let actual_value = request.fields().get(k);

            let actual_value = match expected_value {
                FieldValue::Any => actual_value.map(|_| true),
                FieldValue::Exact(value) => actual_value.map(|v| *value == *v),
            };

            if !actual_value.unwrap_or(false) {
                return false;
            }
        }

        true
    }

    pub fn call(&self, request: Req) -> HandlerFuture<Resp> {
        self.handler.call(request)
    }
}

use std::collections::HashMap;

/// The request type
///
///
#[derive(Debug, Default)]
pub struct TestRequest {
    code: u32,
    fields: HashMap<u32, u32>,
}

impl TestRequest {
    pub fn new(code: u32) -> Self {
        Self {
            code,
            fields: HashMap::new(),
        }
    }

    pub fn with_fields(code: u32, fields: Vec<(u32, u32)>) -> Self {
        Self {
            code,
            fields: fields.into_iter().collect(),
        }
    }
}

pub trait Request {
    fn code(&self) -> u32;
    fn fields(&self) -> &HashMap<u32, u32>;
}

impl Request for TestRequest {
    fn code(&self) -> u32 {
        self.code
    }
    fn fields(&self) -> &HashMap<u32, u32> {
        &self.fields
    }
}


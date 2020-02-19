use crate::handler::HandlerFuture;

/// The response type
///
///
#[derive(Debug, Default)]
pub struct Response {}

pub type ResponseFuture = HandlerFuture<Response>;

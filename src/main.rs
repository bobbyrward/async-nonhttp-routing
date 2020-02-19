mod handler;
mod request;
mod response;
mod route;
mod router;

use anyhow::Result;
use request::TestRequest;
use response::{Response, ResponseFuture};
use router::Router;

async fn handle_one_field_one_is_two(request: TestRequest) -> Result<Response> {
    println!("Message 1, field 1 = 2: {:?}", request);
    Ok(Response {})
}

async fn handle_one_field_two_exists(request: TestRequest) -> Result<Response> {
    println!("Message 1, field 2 exists: {:?}", request);
    Ok(Response {})
}

async fn handle_two(request: TestRequest) -> Result<Response> {
    println!("Message 2: {:?}", request);
    Ok(Response {})
}

async fn handle_three(request: TestRequest) -> Result<Response> {
    println!("Message 3: {:?}", request);
    Ok(Response {})
}

async fn handle_default(request: TestRequest) -> Result<Response> {
    println!("Default: {:?}", request);
    Ok(Response {})
}

// struct TestContext;

/// Main
///
///
#[tokio::main]
async fn main() -> Result<()> {
    let mut router: Router<TestRequest, Response> = Router::new();

    router.default(|request| -> ResponseFuture { Box::pin(handle_default(request)) });

    #[rustfmt::skip]
    router
        .route(1)
        .field(1).exact(2)
        .build(|request| -> ResponseFuture { Box::pin(handle_one_field_one_is_two(request)) });

    #[rustfmt::skip]
    router
        .route(1)
        .field(2).exists()
        .build(|request| -> ResponseFuture { Box::pin(handle_one_field_two_exists(request)) });

    router
        .route(2)
        .build(|request| -> ResponseFuture { Box::pin(handle_two(request)) });

    router
        .route(3)
        .build(|request| -> ResponseFuture { Box::pin(handle_three(request)) });

    router.request(TestRequest::new(1))?.await?;
    router
        .request(TestRequest::with_fields(1, vec![(1, 2)]))?
        .await?;
    router
        .request(TestRequest::with_fields(1, vec![(1, 1)]))?
        .await?;
    router
        .request(TestRequest::with_fields(1, vec![(2, 3)]))?
        .await?;
    router.request(TestRequest::new(2))?.await?;
    router.request(TestRequest::new(3))?.await?;
    router.request(TestRequest::new(4))?.await?;

    Ok(())
}

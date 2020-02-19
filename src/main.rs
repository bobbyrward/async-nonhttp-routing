mod handler;
mod request;
mod response;
mod route;
mod router;

use anyhow::Result;
use request::TestRequest;
use response::Response;
use route::FieldValue;
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

/// Main
///
///
#[tokio::main]
async fn main() -> Result<()> {
    let mut router: Router<TestRequest, Response> = Router::new();

    router.default(handle_default);

    router
        .route(1)
        .field(1, FieldValue::Exact(2))
        .field(2, FieldValue::Exact(5))
        .field(3, FieldValue::Exact(1231))
        .field(7, FieldValue::Exact(212_312))
        .field(10, FieldValue::Exact(208))
        .build(handle_one_field_one_is_two);

    router
        .route(1)
        .field(1, FieldValue::Exact(2))
        .build(handle_one_field_one_is_two);

    router
        .route(1)
        .field(2, FieldValue::Any)
        .build(handle_one_field_two_exists);

    router.route(2).build(handle_two);
    router.route(3).build(handle_three);

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

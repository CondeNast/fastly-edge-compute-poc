use fastly::http::StatusCode;
use fastly::{Body, Response};

pub fn get_root() -> Result<Response<Body>, fastly::http::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Welcome to Fastly Compute@Edge!"))
}

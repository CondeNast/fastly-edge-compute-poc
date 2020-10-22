use fastly::http::{Method, StatusCode};
use fastly::{Body, Error, Request, Response, ResponseExt};
use std::convert::TryFrom;

#[fastly::main]
fn main(req: Request<Body>) -> Result<impl ResponseExt, Error> {
    match req.method() {
        &Method::GET => match req.uri().path() {
            "/" => Ok(Response::try_from(my_app::render_root()?)?),
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("The page you requested could not be found")?),
        },
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body("This method is not allowed")?),
    }
}

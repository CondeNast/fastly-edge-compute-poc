use fastly::http::{Method, StatusCode};
use fastly::{Body, Error, Request, Response, ResponseExt};

mod routes;

#[fastly::main]
fn main(req: Request<Body>) -> Result<impl ResponseExt, Error> {
    if req.method() != Method::GET {
        Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("This method is not allowed"))?)
    } else {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => Ok(routes::get_root()?),
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("The page you requested could not be found"))?),
        }
    }
}

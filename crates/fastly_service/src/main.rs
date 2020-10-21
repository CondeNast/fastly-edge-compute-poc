use fastly::http::{Method, StatusCode};
use fastly::{Body, Error, Request, Response, ResponseExt};

fn fastly_response_from_std(res: http::Response<&str>) -> fastly::Response<fastly::Body> {
    Response::builder()
        .status(res.status())
        .body(Body::from(*res.body()))
        .unwrap() // @FIXME: handle errors or bubble result up to calling function
}

#[fastly::main]
fn main(req: Request<Body>) -> Result<impl ResponseExt, Error> {
    if req.method() != Method::GET {
        Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("This method is not allowed"))?)
    } else {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => Ok(fastly_response_from_std(my_app::get_root()?)),
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("The page you requested could not be found"))?),
        }
    }
}

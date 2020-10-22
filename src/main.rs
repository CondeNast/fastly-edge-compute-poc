use fastly::http::{Method, StatusCode};
use fastly::{Body, Error, Request, Response, ResponseExt};

fn fastly_response_from_std(res: http::Response<&str>) -> Result<Response<Body>, Error> {
    match Response::builder()
        .status(res.status())
        .body(Body::from(*res.body()))
    {
        Ok(res) => Ok(res),
        Err(error) => Err(Error::from(error)),
    }
}

#[fastly::main]
fn main(req: Request<Body>) -> Result<impl ResponseExt, Error> {
    match req.method() {
        &Method::GET => match req.uri().path() {
            "/" => Ok(fastly_response_from_std(my_app::render_root()?)?),
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("The page you requested could not be found"))?),
        },
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("This method is not allowed"))?),
    }
}

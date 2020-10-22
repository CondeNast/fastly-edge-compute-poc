use http::{Method, Request, Response, StatusCode};

pub fn render(req: Request<String>) -> Result<Response<String>, http::Error> {
    match req.method() {
        &Method::GET => match req.uri().path() {
            "/" => Ok(Response::builder()
                .status(StatusCode::OK)
                .body(String::from("Welcome to Stef's Computer On The Edge v4!"))?),
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(String::from("The page you requested could not be found"))?),
        },
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(String::from("This method is not allowed"))?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_responds_with_appropriate_body() {
        assert_eq!(
            render(
                Request::builder()
                    .method("GET")
                    .body(String::from("Hello World!"))
                    .unwrap()
            )
            .unwrap()
            .body(),
            &"Welcome to Stef's Computer On The Edge v4!"
        );
    }
}

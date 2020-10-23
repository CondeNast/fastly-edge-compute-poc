use http::{Method, Request, Response, StatusCode};

pub fn render(req: Request<String>) -> Result<Response<&'static str>, http::Error> {
    match req.method() {
        &Method::GET => match req.uri().path() {
            "/" => Ok(Response::builder()
                .status(StatusCode::OK)
                .body("Hello")?),
            "/panic" => panic!("You asked for it"),
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("The page you requested could not be found")?),
        },
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body("This method is not allowed")?),
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
                    .uri(http::Uri::from_static("/"))
                    .method("GET")
                    .body(String::from(""))
                    .unwrap()
            )
            .unwrap()
            .body(),
            &"Welcome to Stef's Computer On The Edge v4!"
        );
    }

    #[test]
    #[should_panic(expected = "You asked for it")]
    fn it_should_panic_when_you_ask_for_it() {
        let _res = render(
            Request::builder()
                .uri(http::Uri::from_static("/panic"))
                .method("GET")
                .body(String::from(""))
                .unwrap(),
        );
    }
}

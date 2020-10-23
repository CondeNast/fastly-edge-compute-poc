use http::{Error, Method, Request, Response, StatusCode};

pub fn render(req: Request<String>) -> Result<Response<String>, Error> {
    match req.method() {
        &Method::GET => match req.uri().path() {
            "/" => Response::builder()
                .status(StatusCode::OK)
                .body(String::from("Welcome to Stef's Computer On The Edge v4!")),
            "/panic" => panic!("You asked for it"),
            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(String::from("The page you requested could not be found")),
        },
        _ => Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(String::from("This method is not allowed")),
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

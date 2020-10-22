use http::{Response, StatusCode};

pub fn render_root() -> Result<Response<&'static str>, http::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body("Welcome to Stef's Computer On The Edge v3!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_responds_with_appropriate_body() {
        assert_eq!(
            render_root().unwrap().body(),
            &"Welcome to Stef's Computer On The Edge v3!"
        );
    }
}

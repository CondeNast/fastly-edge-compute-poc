use http::{Response, StatusCode};

pub fn get_root() -> Result<Response<&'static str>, http::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body("Welcome to Stef's Computer On The Edge v3!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_responds_with_appropriate_body() {
        let result = *get_root().unwrap().body();
        assert_eq!(result, "Welcome to Stef's Computer On The Edge v3!");
    }
}

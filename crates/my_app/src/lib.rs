use http::{Response, StatusCode};

pub fn get_root() -> Result<Response<&'static str>, http::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body("Welcome to Fastly Compute@Edge!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_responds_with_ok() {
        let result = *get_root().unwrap().body();
        assert_eq!(result, "Welcome to Fastly Compute@Edge!");
    }
}

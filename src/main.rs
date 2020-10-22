use fastly::ResponseExt;

#[fastly::main]
fn main(req: fastly::Request<fastly::Body>) -> Result<impl fastly::ResponseExt, fastly::Error> {
    let (parts, body) = req.into_parts();
    let req = http::Request::from_parts(parts, body.into_string());
    let res = my_app::render(req);
    match res {
        Ok(res) => Ok(res),
        Err(error) => Err(fastly::Error::from(error)),
    }
}

use fastly::ResponseExt;
use std::convert::TryFrom;

#[fastly::main]
fn main(req: fastly::Request<fastly::Body>) -> Result<impl fastly::ResponseExt, fastly::Error> {
    let (parts, body) = req.into_parts();
    let req = http::Request::from_parts(parts, body.into_string());
    let res = my_app::render(req)?;
    Ok(fastly::Response::try_from(res)?)
}

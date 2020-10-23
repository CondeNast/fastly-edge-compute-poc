use fastly::ResponseExt;

#[fastly::main]
fn main(req: fastly::Request<fastly::Body>) -> Result<impl fastly::ResponseExt, fastly::Error> {
    // convert fastly::Request to standard http::Request
    let (parts, body) = req.into_parts();
    let req = http::Request::from_parts(parts, body.into_string());

    // render app
    let res: http::Response<String> = app::render(req)?;

    // convert response back to fastly::Response
    let res: fastly::Response<String> = res.into();
    Ok(res)
}

use iron::{status, error::IronError, headers::ContentType, modifiers::Header,
           prelude::*};

pub fn handler(req: &mut Request) -> Result<Response, IronError> {
    Ok(Response::with((
        status::Ok,
        Header(ContentType::html()),
        include_str!("index.html"),
    )))
}

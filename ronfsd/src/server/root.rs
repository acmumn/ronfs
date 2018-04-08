use iron::{status, error::IronError, prelude::*};

/// The handler funtion for the `/` path.
pub fn handler(req: &mut Request) -> Result<Response, IronError> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

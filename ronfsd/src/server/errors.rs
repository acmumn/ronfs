use iron::{status, AfterMiddleware, error::IronError, prelude::*};

/// The middleware for errors.
pub struct Errors;

impl AfterMiddleware for Errors {
    fn catch(
        &self,
        req: &mut Request,
        err: IronError,
    ) -> Result<Response, IronError> {
        match err.response.status {
            Some(status::Ok) => {
                Ok(Response::with((status::Ok, "Hello World!")))
            }
            Some(s) => Ok(Response::with((s, "Hello World!"))),
            None => Ok(Response::with((status::Ok, "Hello World!"))),
        }
    }
}

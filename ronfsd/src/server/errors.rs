use either::{Either, Left, Right};
use iron::{status, AfterMiddleware, error::IronError, headers::ContentType,
           modifiers::Header, prelude::*};

const TEMPLATE: &str = include_str!("error.html");

/// The middleware for errors.
pub struct Errors;

impl AfterMiddleware for Errors {
    fn catch(
        &self,
        req: &mut Request,
        err: IronError,
    ) -> Result<Response, IronError> {
        let status = err.response.status.unwrap_or(status::InternalServerError);
        let replacement = status.to_string();
        let body = TEMPLATE.replace("{{}}", &html_escape(&replacement));
        Ok(Response::with((status, Header(ContentType::html()), body)))
    }
}

// As per https://www.owasp.org/index.php/XSS_(Cross_Site_Scripting)_Prevention_Cheat_Sheet#RULE_.231_-_HTML_Escape_Before_Inserting_Untrusted_Data_into_HTML_Element_Content
fn html_escape(s: &str) -> String {
    let escch = |c| match c {
        '&' => Right("&amp;"),
        '<' => Right("&lt;"),
        '>' => Right("&gt;"),
        '"' => Right("&quot;"),
        '\'' => Right("&#x27;"),
        '/' => Right("&#x2F;"),
        c => Left(c),
    };

    let len = s.chars()
        .map(|c| match escch(c) {
            Left(c) => c.len_utf8(),
            Right(s) => s.len(),
        })
        .sum();

    let mut out = String::with_capacity(len);
    for c in s.chars() {
        match escch(c) {
            Left(ch) => out.push(ch),
            Right(str) => out.push_str(str),
        }
    }
    debug_assert_eq!(len, out.len());
    out
}

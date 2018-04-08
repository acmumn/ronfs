pub mod errors;
pub mod root;

use std::path::PathBuf;

use iron::typemap::Key;

/// The root path.
pub enum RootPath {}

impl Key for RootPath {
    type Value = PathBuf;
}

#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

/// Metadata about a single file.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileInfo {
    // TODO
}

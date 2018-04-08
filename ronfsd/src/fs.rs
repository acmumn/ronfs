use std::io::Error as IoError;
use std::path::Path;

/// Returns whether p2 is a path within p1.
pub fn path_within<P1: AsRef<Path>, P2: AsRef<Path>>(p1: P1, p2: P2) -> bool {
    let f = move || -> Result<bool, IoError> {
        let p1 = p1.as_ref().canonicalize()?;
        let p2 = p2.as_ref().canonicalize()?;

        // TODO

        Ok(true)
    };
    f().unwrap_or(false)
}

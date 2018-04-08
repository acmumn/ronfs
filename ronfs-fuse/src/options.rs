use std::path::PathBuf;

use hyper::Uri;

/// A FUSE filesystem for accessing files in a read-only way over the network.
#[derive(Debug, StructOpt)]
pub struct Options {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// The UID of the user who owns the files.
    #[structopt(long = "uid")]
    pub uid: Option<u32>,

    /// The GID of the group who owns the files.
    #[structopt(long = "gid")]
    pub gid: Option<u32>,

    /// The permissions applied to the files. The default value corresponds to
    /// `0o555`.
    #[structopt(short = "p", long = "permissions", default_value = "365")]
    pub permissions: u16,

    /// The URI of the RonFS server.
    pub server_uri: Uri,

    /// The mountpoint.
    #[structopt(parse(from_os_str))]
    pub mountpoint: PathBuf,
}

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

    /// The URI of the RonFS server.
    pub server_uri: Uri,

    /// The mountpoint.
    #[structopt(parse(from_os_str))]
    pub mountpoint: PathBuf,
}

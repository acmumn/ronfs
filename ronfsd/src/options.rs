use std::net::IpAddr;
use std::path::PathBuf;

/// A FUSE filesystem for accessing files in a read-only way over the network.
#[derive(Debug, StructOpt)]
pub struct Options {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// The interface to serve on.
    #[structopt(long = "interface", default_value = "0.0.0.0")]
    pub interface: IpAddr,

    /// The port to serve on.
    #[structopt(short = "p", long = "port", default_value = "8000")]
    pub port: u16,

    /// The path to serve.
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
}

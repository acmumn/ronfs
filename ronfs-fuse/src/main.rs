extern crate failure;
extern crate fuse;
extern crate hyper;
extern crate libc;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate time;

mod fs;
mod options;

use failure::Error;
use structopt::StructOpt;

use fs::Filesystem;
use options::Options;

fn main() {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose)
        .init()
        .unwrap();

    if let Err(err) = run(options) {
        error!("{}", err);
    }
}

fn run(options: Options) -> Result<(), Error> {
    let fs = Filesystem::new(options.server_uri);
    fuse::mount(fs, &options.mountpoint, &[])?;
    Ok(())
}

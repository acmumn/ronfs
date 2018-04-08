extern crate chan_signal;
extern crate failure;
extern crate fuse;
extern crate futures;
extern crate hyper;
extern crate libc;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate time;
extern crate tokio_core;
extern crate users;

mod client;
mod fs;
mod options;

use failure::Error;
use hyper::Client;
use structopt::StructOpt;
use tokio_core::reactor::Core;

use fs::Filesystem;
use options::Options;

fn main() {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose)
        .init()
        .unwrap();

    debug!("Got options {:#?}", options);
    if let Err(err) = run(options) {
        error!("{}", err);
    }
}

fn run(options: Options) -> Result<(), Error> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let fs = Filesystem::new(client, options.server_uri);
    let session = fuse::mount(fs, &options.mountpoint, &[])?;
    Ok(())
}

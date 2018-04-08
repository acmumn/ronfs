extern crate failure;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;
extern crate persistent;
extern crate stderrlog;
#[macro_use]
extern crate structopt;

mod options;
mod server;

use failure::Error;
use iron::{Chain, Iron};
use logger::Logger;
use mount::Mount;
use persistent::Read;
use structopt::StructOpt;

use options::Options;
use server::{root, RootPath, errors::Errors};

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
    let mut mount = Mount::new();

    let mut chain = Chain::new(mount);
    chain.link(Logger::new(None));
    chain.link_before(Read::<RootPath>::one(options.path));
    chain.link_after(Errors);

    info!("Starting...");
    let _ = Iron::new(chain).http((options.interface, options.port))?;
    info!("Stopping...");
    Ok(())
}

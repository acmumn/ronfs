extern crate either;
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

mod fs;
mod options;
mod server;

use failure::Error;
use iron::{Chain, Handler, Iron, Request};
use logger::Logger;
use mount::Mount;
use persistent::Read;
use structopt::StructOpt;

use options::Options;
use server::{RootPath, errors::Errors, root::handler as root_handler};

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

    let mut chain = Chain::new(move |req: &mut Request| {
        let is_root = {
            let url: &iron::url::Url = req.url.as_ref();
            url.path() == "/"
        };
        if is_root {
            root_handler(req)
        } else {
            mount.handle(req)
        }
    });
    chain.link(Logger::new(None));
    chain.link_before(Read::<RootPath>::one(options.path));
    chain.link_after(Errors);

    info!("Starting...");
    let _ = Iron::new(chain).http((options.interface, options.port))?;
    info!("Stopping...");
    Ok(())
}

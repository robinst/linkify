// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;

use structopt::StructOpt;
use std::path::PathBuf;

/// Extract, normalize, and clean data from various sources.
#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    if opt.verbose == 2 {
        println!("{:#?}", opt);
    }

    if let Err(ref e) = run(&opt) {
        use error_chain::ChainedError;
        use std::io::Write; // trait which holds `display_chain`
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run(options: &Opt) -> Result<()> {
  use std::io::{self, Read};
  use url::Url;

  let mut buffer = String::new();
  let stdin = io::stdin();
  stdin.lock().read_to_string(&mut buffer).chain_err(|| "could not read input")?;

  let links: Links = find_links(&buffer).expect("could not get links");

  for link in links {
    let link_str = link.as_str();
    if let Ok(_url) = Url::parse(&link_str) {
        println!("{}", &link_str);
    } else {
        eprintln!("Could NOT parse: {}", &link_str);
    }
  }

  Ok(())
}

extern crate linkify;
use linkify::Links;
use linkify::LinkFinder;

fn find_links<'t>(string: &'t String) -> Result<Links<'t>> {
    let finder = LinkFinder::new();
    Ok(finder.links(&string))
}

extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;


fn main() {
    // Ok has a value => unwrap
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Ok has no value => handle err
    if let Err(e) = minigrep::run(config) {
        eprint!("Application error: {}", e);
        process::exit(1);
    }
}

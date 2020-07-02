use std::env;
use std::io;
use std::process;
use myminigrep::config::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    // pass by reference, so run does not own config struct
    myminigrep::run(&config);
}

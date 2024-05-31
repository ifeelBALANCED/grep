use std::env;
use std::process;

use grep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grep::run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
}
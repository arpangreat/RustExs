use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguements: {}", err);
        process::exit(1);
    });

    println!("{}", b);

    //    println!("Searching for {}", config.query);

    //    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        // Call Process for exit

        process::exit(1);
    }
}

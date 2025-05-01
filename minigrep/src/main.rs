use std::env;
use std::process;

use minigrep::Config;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

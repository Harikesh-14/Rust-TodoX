use std::env;
use todo::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
       eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = todo::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}
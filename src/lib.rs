use std::error::Error;

pub struct Config {
    operation: String,
    tasks: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            println!("Use at least 2 arguments")
        }
        let operation = args[1].clone();
        let tasks = args.get(2).cloned().unwrap_or_else(|| String::from("None"));

        Ok(Config { operation, tasks })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.operation.as_str() {
        "add" => {

        }

        _ => {
            println!("Invalid Operations");
        }
    }

    Ok(())
}
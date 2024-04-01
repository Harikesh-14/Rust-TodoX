mod conn;

use std::error::Error;
use rusqlite::Connection;

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
    let conn = Connection::open("user.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            task_name TEXT NOT NULL,
            date_added TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;

    match config.operation.as_str() {
        "add" => {
            conn::insert_task(&conn, &config.tasks)?;
        }

        "show" => {
            conn::display_tasks(&conn)?;
        }

        _ => {
            println!("Invalid Operations");
        }
    }

    Ok(())
}
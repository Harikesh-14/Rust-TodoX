mod conn;

use std::error::Error;
use std::io;
use rusqlite::Connection;

pub struct Config {
    operation: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            println!("Use at least 2 arguments")
        }
        let operation = args[1].clone();

        Ok(Config { operation })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("user.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            task_name TEXT NOT NULL,
            date_added TEXT
        )",
        (),
    )?;

    match config.operation.as_str() {
        "add" => {
            let mut task_name = String::new();
            println!("Enter the task name: ");
            io::stdin()
                .read_line(&mut task_name)
                .expect("Expected an input");
            let task_name = task_name.trim();

            if conn::is_present(&conn, &task_name) == false {
                conn::insert_task(&conn, &task_name)?;
            } else {
                eprintln!("This task already exist.");
            }
        }
        "delete" => {
            let mut task_name = String::new();
            println!("Enter the task name: ");
            io::stdin()
                .read_line(&mut task_name)
                .expect("Expected an input");
            let task_name = task_name.trim();

            if conn::is_present(&conn, &task_name) {
                conn::delete_task(&conn, &task_name)?;
            } else {
                eprintln!("The task is not present in the todo.")
            }
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
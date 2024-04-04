mod conn;

use std::error::Error;
use std::io;
use rusqlite::Connection;
use colored::*;

pub struct Config {
    pub operation: String,
    pub task_id: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            println!("{}", "Use at least 2 arguments".red())
        }
        let operation = args[1].clone();
        let task_id = args.get(2).cloned().unwrap_or_else(|| String::from("None"));

        Ok(Config { operation, task_id })
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

            if conn::is_present_name(&conn, &task_name) == false {
                conn::insert_task(&conn, &task_name)?;
                println!("{}", "Task added".green());
            } else {
                eprintln!("{}", "This task already exist.".red());
            }
        }
        "done" => {
            let task_id = config.task_id;
            let task_id: i32 = task_id.trim().parse().expect("Please enter a valid task ID");

            if conn::is_present(&conn, task_id) {
                conn::delete_task(&conn, task_id)?;
                println!("{}", "Task mark as done!".red());
            } else {
                eprintln!("Task with ID {} not found.", task_id);
            }
        }
        "show" => {
            match conn::is_table_empty(&conn) {
                Ok(task) => {
                    if task == false {
                        conn::display_tasks(&conn)?;
                    } else {
                        println!("{}", "Todo list is empty!".red());
                    }
                },
                Err(_) => println!("Some error occurred"),
            }
        }
        "edit" => {
            let task_id = config.task_id;
            let task_id: i32 = task_id.trim().parse().expect("Please enter a valid task ID");

            let mut new_task_name = String::new();
            println!("Enter the new task name: ");
            io::stdin()
                .read_line(&mut new_task_name)
                .expect("Expected an input");
            let new_task_name = new_task_name.trim();

            if conn::is_present(&conn, task_id) {
                conn::update_task_name(&conn, task_id, new_task_name)?;
                println!("{}", "Task name updated!".green());
            } else {
                eprintln!("Task with ID {} not found.", task_id);
            }
        }
        "clear" => {
            match conn::is_table_empty(&conn) {
                Ok(task) => {
                    if task == false {
                        if config.task_id == "all" {
                            conn::clear_table(&conn)?;
                            println!("{}", "Todo list cleared!".green());
                        } else {
                            println!("Did you mean: `clear all`?");
                        }
                    } else {
                        println!("{}", "Todo list is already empty!".red());
                    }
                },
                Err(_) => println!("Some error occurred"),
            }
        }
        _ => {
            println!("{}", "Invalid Operations".red());
        }
    }

    Ok(())
}
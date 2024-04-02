use rusqlite::{Connection, Error, Result};
use chrono::Local;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Person {
    id: i32,
    task_name: String,
    date_added: String,
}

pub fn is_present(conn: &Connection, task_id: i32) -> bool {
    let mut stmt = conn.prepare("SELECT * FROM task WHERE id = ?").expect("Error preparing statement");
    let task_iter = stmt.query_map(&[&task_id], |_| Ok(()))
        .expect("Error querying tasks");

    for _ in task_iter {
        return true;
    }

    false
}

pub fn is_present_name(conn: &Connection, task_name: &str) -> bool {
    let mut stmt = conn.prepare("SELECT * FROM task WHERE task_name = ?").expect("Error preparing statement");
    let task_iter = stmt.query_map(&[&task_name], |row| {
        Ok(Person {
            id: row.get(0)?,
            task_name: row.get(1)?,
            date_added: row.get(2)?,
        })
    }).expect("Error querying tasks");

    for t in task_iter {
        if let Ok(task) = t {
            if task.task_name == task_name {
                return true;
            }
        }
    }

    false
}

pub fn insert_task(conn: &Connection, task_name: &str) -> Result<(), Error> {
    let curr_date = Local::now();
    let formatted_date = curr_date.format("%d-%m-%Y %H-%M-%S").to_string();

    conn.execute(
        "INSERT INTO task (task_name, date_added) VALUES (?, ?)",
        &[task_name, &formatted_date],
    )?;
    Ok(())
}

pub fn display_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            task_name: row.get(1)?,
            date_added: row.get(2)?,
        })
    })?;

    for t in task_iter {
        match t {
            Ok(task) => {
                let curr_id = task.id;
                let curr_task = task.task_name;
                let curr_date = task.date_added;

                println!("[{}]. {} --> ({})", curr_id ,curr_task, curr_date);
            }

            Err(err) => {
                eprintln!("Error processing task: {}", err);
            }
        }
    }

    Ok(())
}

pub fn delete_task(conn: &Connection, task_id: i32) -> Result<()> {
    conn.execute("DELETE FROM task WHERE id = ?", [task_id])?;

    // After deleting the task, update the IDs of remaining tasks
    conn.execute("UPDATE task SET id = id - 1 WHERE id > ?", [task_id])?;

    Ok(())
}
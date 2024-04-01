use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Person {
    id: i32,
    task_name: String,
    date_added: String,
}

pub fn insert_task(conn: &Connection, task_name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO task (task_name) VALUES (?1)",
        &[task_name],
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
        println!("Your tasks: {:?}", t.unwrap())
    }

    Ok(())
}

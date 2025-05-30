use crate::Task;
use rusqlite::{params, Connection, Error as SqliteError};

pub struct TaskManager {
    conn: Connection,
}

impl TaskManager {

    pub fn new(conn: Connection) -> Self {
        TaskManager { conn }
    }

    pub fn add_task(&self, task: Task) -> Result<(), SqliteError> {
        self.conn.execute(
            "INSERT INTO task (id, title, completed) VALUES (?1, ?2, ?3)",
            params![&task.id, &task.title, &task.completed],
        )?;

        Ok(())
    }

    pub fn view_tasks(&self) -> Result<(), SqliteError> {
        let mut stmt = self.conn.prepare("SELECT title, completed FROM task")?;
        let task_iter = stmt.query_map([], |row| {
            let title: String = row.get(0)?;
            let completed: bool = row.get(1)?;

            Ok((title, completed))
        })?;

        for task in task_iter {
            let (title, completed) = task?;
            println!("{} [{}]", title, if completed { "V" } else { " " });
        }

        Ok(())
    }
}

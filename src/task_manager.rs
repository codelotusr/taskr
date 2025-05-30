use crate::Task;
use rusqlite::{Connection, Error as SqliteError, params};

pub struct TaskManager {
    conn: Connection,
}

impl TaskManager {
    pub fn add_task(&self, task: Task) -> Result<(), SqliteError> {
        self.conn.execute(
            "INSERT INTO task (id, title, due_date, completed) VALUES (?1, ?2, ?3, ?4)",
            params![&task.id, &task.title, &task.due_date, &task.completed],
        )?;

        Ok(())
    }
}

use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub due_date: NaiveDate,
    pub completed: bool,
}


impl Task {
    pub fn new(title: String, due_date: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title,
            due_date: due_date,
            completed: false,
        }
    }
}

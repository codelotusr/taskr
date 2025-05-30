use chrono::NaiveDate;
use uuid::Uuid;

pub struct Task {
    id: Uuid,
    title: String,
    due_date: NaiveDate,
    completed: bool,
}

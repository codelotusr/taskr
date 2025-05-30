use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title,
            completed: false,
        }
    }
}

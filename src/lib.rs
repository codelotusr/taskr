pub mod task;
pub mod storage;
pub mod tasks_manager;

pub fn run<I: Iterator<Item=String>>(_args: I) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}


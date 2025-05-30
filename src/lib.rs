pub mod storage;
pub mod task;
pub mod task_manager;
pub use task::Task;
pub use task_manager::TaskManager;

pub fn run<I: Iterator<Item = String>>(_args: I) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

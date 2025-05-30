use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::{storage, Task, TaskManager};

#[derive(Parser)]
#[command(name = "taskr", about = "CLI to-do list")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { title: String },
    List,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let conn = Connection::open("taskrdb.sqlite")?;
    storage::init_db(&conn)?;
    let manager = TaskManager::new(conn);

    match cli.command {
        Commands::Add { title } => {
            let task = Task::new(title);
            manager.add_task(task)?;
        }
        Commands::List => {
            manager.view_tasks()?;
        }
    }

    Ok(())
}

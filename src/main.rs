mod cli;
mod models;
mod storage;

use cli::args::{Cli, Commands};
use models::task::TaskList;
use storage::{load_from_file, save_to_file};
use anyhow::Result;
use clap::Parser; 

fn main() -> Result<()> {
    let cli = Cli::parse();
    let filename = "tasks.json";
    // Use storage::load_from_file instead of TaskList::load_from_file
    let mut task_list = load_from_file(filename).unwrap_or_else(|_| TaskList::new());

    match cli.command {
        Commands::Add { description } => {
            let id = task_list.tasks.len() as u32 + 1;
            task_list.add_task(id, description);
            println!("Added task: {}", id);
        }
        Commands::List => task_list.list_tasks(),
        Commands::Complete { id } => task_list.complete_task(id),
        Commands::Delete { id } => task_list.delete_task(id),
    }

    save_to_file(&task_list, filename)?;
    Ok(())
}
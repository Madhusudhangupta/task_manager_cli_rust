use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Task Manager")]
#[command(about = "A simple CLI task manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add { description: String },
    /// List all tasks
    List,
    /// Mark a task as complete
    Complete { id: u32 },
    /// Delete a task
    Delete { id: u32 },
}
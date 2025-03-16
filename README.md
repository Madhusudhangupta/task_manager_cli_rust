# Rust Task Manager CLI

A command-line task manager built in Rust, featuring persistent storage with JSON, command-line parsing with `clap`, and robust error handling with `anyhow`.

## Features
- Add tasks with descriptions
- List all tasks with completion status
- Mark tasks as complete
- Delete tasks
- Persists tasks to `tasks.json`

## Usage
```bash
cargo run -- add "Your task here"  # Add a task
cargo run -- list                 # List tasks
cargo run -- complete <id>        # Complete a task by ID
cargo run -- delete <id>          # Delete a task by ID
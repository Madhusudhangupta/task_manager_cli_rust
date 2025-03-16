use std::fs;
use serde_json;
use crate::models::task::TaskList;
use anyhow::Result;

pub fn save_to_file(task_list: &TaskList, filename: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(task_list)?;
    fs::write(filename, json)?;
    Ok(())
}

pub fn load_from_file(filename: &str) -> Result<TaskList> {
    let json = fs::read_to_string(filename)?;
    let list = serde_json::from_str(&json)?;
    Ok(list)
}
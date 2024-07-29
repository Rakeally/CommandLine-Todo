use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: usize, title: &str, description: &str) -> Self {
        Task {
            id,
            title: title.to_string(),
            description: description.to_string(),
            status: Status::InProgress,
        }
    }
}

const FILE_NAME: &str = "tasks.json";

pub fn read_tasks() -> Result<Vec<Task>, Error> {
    if !Path::new(FILE_NAME).exists() {
        return Ok(Vec::new());
    }

    //read file content
    let mut file = OpenOptions::new().read(true).open(FILE_NAME).unwrap();
    let mut data = String::new();
    //read file content to string and save in data
    file.read_to_string(&mut data).unwrap();
    //deserialize string to json of Task structs
    serde_json::from_str(&data)
}

pub fn write_tasks(tasks: &Vec<Task>) -> Result<(), Error> {
    let data = serde_json::to_string(tasks)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_NAME)
        .unwrap();
    file.write_all(data.as_bytes()).unwrap();
    Ok(())
}

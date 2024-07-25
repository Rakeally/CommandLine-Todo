mod task;

use serde_json::Error;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};
use task::Task;

const FILE_NAME: &str = "tasks.json";

fn read_task() -> Result<Vec<Task>, Error> {
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

fn write_task(tasks: &Vec<Task>) -> Result<(), Error> {
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

fn main() {}

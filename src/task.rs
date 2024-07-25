use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: usize,
    title: String,
    description: String,
    status: Status,
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

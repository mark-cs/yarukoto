use std::{
    collections::HashSet,
    fs,
    hash::{Hash, Hasher},
    io::Error,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

/// A task
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Task {
    title: String,
    priority: Option<Priority>,
    status: Option<Status>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    tags: HashSet<String>,
}

impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.priority.hash(state);
        self.status.hash(state);
        self.description.hash(state);
        self.tags.iter().for_each(|t| t.hash(state));
    }
}

impl Task {
    pub fn from_path(path: PathBuf) -> Result<Task, Error> {
        let file_content = fs::read_to_string(path)?;
        match toml::from_str(&file_content) {
            Ok(task) => Ok(task),
            Err(err) => Err(Error::other(err)),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Hash, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Pending,
    InProgress,
    Complete,
}

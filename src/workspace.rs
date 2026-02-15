use std::{io::Error, path::PathBuf};

use crate::task::Task;

#[derive(Debug)]
pub struct Workspace {
    pub name: String,
    path: PathBuf,
}

impl Workspace {
    pub fn new(path: &String) -> Result<Workspace, Error> {
        let path_buf = PathBuf::from(path);
        Ok(Workspace {
            name: String::from(
                path_buf
                    .file_name()
                    .and_then(|s| s.to_str())
                    .ok_or(Error::other("Unable to read workspace name from directory"))?,
            ),
            path: path_buf,
        })
    }

    pub fn read_tasks(&self) -> Result<Vec<Task>, Error> {
        self.path
            .read_dir()?
            .flatten()
            .map(|entry| Task::from_path(entry.path()))
            .collect()
    }
}

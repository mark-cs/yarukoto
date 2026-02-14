use std::io::Error;

use ratatui::widgets::ListState;

use crate::{task::Task, workspace::Workspace};

#[derive(Debug, Default)]
pub struct Model {
    workspaces: Vec<Workspace>,

    pub focussed_panel: Panel,
    pub running_state: RunningState,

    // TUI SelectWorkspace
    workspace_selection: ListState,
    task_selection: ListState,
}

impl Model {
    pub fn workspace_selection_mut(self: &mut Model) -> &mut ListState {
        &mut self.workspace_selection
    }
    pub fn task_selection_mut(self: &mut Model) -> &mut ListState {
        &mut self.task_selection
    }

    pub fn workspace_selection(&self) -> &ListState {
        &self.workspace_selection
    }

    pub fn task_selection(&self) -> &ListState {
        &self.task_selection
    }
    pub fn task_count(&self) -> Result<usize, Error> {
        match self.workspace_selection.selected() {
            Some(index) => Ok(self.workspaces[index].tasks()?.len()),
            None => Ok(0),
        }
    }

    pub(crate) fn workspace_count(&self) -> usize {
        self.workspaces.len()
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }
    pub(crate) fn workspaces_mut(&mut self) -> &mut Vec<Workspace> {
        &mut self.workspaces
    }
    pub fn tasks(&self) -> Result<Vec<Task>, Error> {
        match self.workspace_selection.selected() {
            Some(index) => Ok(self.workspaces[index].tasks()?),
            None => Ok(Vec::new()),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Panel {
    #[default]
    WorkspaceList,
    TaskList,
    Task,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    SelectWorkspace { workspace_index: usize },
    SelectTask { task_index: usize },
    Quit,
    FocusWorkspacePane,
    FocusTaskListPane,
    FocusTaskPane,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

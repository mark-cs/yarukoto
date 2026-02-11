use crate::workspace::Workspace;

#[derive(Debug, Default)]
pub struct Model {
    pub workspaces: Vec<Workspace>,
    pub selected_workspace: Option<i32>,
    pub selected_task: Option<i32>,

    pub focussed_panel: Panel,
    pub running_state: RunningState,
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
    SelectWorkspace { workspace_index: i32 },
    SelectTask { task_index: i32 },
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

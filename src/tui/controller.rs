use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, KeyCode};

use crate::result::Result;
use crate::tui::model::{Action, Model, Panel, RunningState};
use crate::tui::view;

pub struct Controller {
    model: Model,
}

impl Controller {
    pub fn new(model: Model) -> Controller {
        Controller { model }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.model.running_state != RunningState::Done {
            // Render the current view
            terminal.draw(|frame| view::view(frame, &mut self.model))?;
            if let Some(action) = self.handle_event()? {
                self.handle_action(action);
            }
        }
        Ok(())
    }

    fn handle_event(&self) -> Result<Option<Action>> {
        if event::poll(Duration::from_millis(250))?
            && let Some(key_event) = event::read()?.as_key_event()
        {
            return match (key_event.code, &self.model.focussed_panel) {
                (KeyCode::Up, Panel::TaskList) => match self.model.task_selection().selected() {
                    Some(index) => Ok(Some(Action::SelectTask {
                        task_index: index.saturating_sub(1),
                    })),
                    None => Ok(Some(Action::SelectTask { task_index: 0 })),
                },
                (KeyCode::Down, Panel::TaskList) => match self.model.task_selection().selected() {
                    Some(index) => Ok(Some(Action::SelectTask {
                        task_index: std::cmp::min(
                            index.saturating_add(1),
                            self.model.task_count() - 1,
                        ),
                    })),
                    None => Ok(Some(Action::SelectTask { task_index: 0 })),
                },
                (KeyCode::Up, Panel::WorkspaceList) => {
                    match self.model.workspace_selection().selected() {
                        Some(index) => Ok(Some(Action::SelectWorkspace {
                            workspace_index: index.saturating_sub(1),
                        })),
                        None => Ok(Some(Action::SelectWorkspace { workspace_index: 0 })),
                    }
                }
                (KeyCode::Down, Panel::WorkspaceList) => {
                    match self.model.workspace_selection().selected() {
                        Some(index) => Ok(Some(Action::SelectWorkspace {
                            workspace_index: std::cmp::min(
                                index.saturating_add(1),
                                self.model.workspace_count() - 1,
                            ),
                        })),
                        None => Ok(Some(Action::SelectWorkspace { workspace_index: 0 })),
                    }
                }
                (KeyCode::Char('w'), _) => Ok(Some(Action::FocusWorkspacePane)),
                (KeyCode::Char('l'), _) => Ok(Some(Action::FocusTaskListPane)),
                (KeyCode::Char('t'), _) => Ok(Some(Action::FocusTaskPane)),
                (KeyCode::Char('q'), _) => Ok(Some(Action::Quit)),
                _ => Ok(None),
            };
        }
        Ok(None)
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.model.running_state = RunningState::Done,
            Action::FocusWorkspacePane => self.model.focussed_panel = Panel::WorkspaceList,
            Action::FocusTaskListPane => self.model.focussed_panel = Panel::TaskList,
            Action::FocusTaskPane => self.model.focussed_panel = Panel::Task,
            Action::SelectWorkspace { workspace_index } => {
                self.model
                    .workspace_selection_mut()
                    .select(Some(workspace_index));
                self.model.task_selection_mut().select(Some(0));
                self.model.replace_tasks(
                    self.model.workspaces()[workspace_index]
                        .read_tasks()
                        .unwrap_or_default(),
                );
            }
            Action::SelectTask { task_index } => {
                self.model.task_selection_mut().select(Some(task_index))
            }
        }
    }
}

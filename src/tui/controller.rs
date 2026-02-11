use std::time::Duration;

use ratatui::crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;

use crate::result::Result;
use crate::tui::model::{Action, Model, Panel, RunningState};

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
            terminal.draw(|frame| self.view(frame))?;

            if let Some(action) = self.handle_event()? {
                self.handle_action(action);
            }
        }

        Ok(())
    }
    fn view(&self, frame: &mut ratatui::Frame<'_>) {
        frame.render_widget("hello world", frame.area());
    }
    fn handle_event(&self) -> Result<Option<Action>> {
        if event::poll(Duration::from_millis(250))? {
            let event = event::read()?;
            if let Some(key_event) = event.as_key_event() {
                return match key_event.code {
                    KeyCode::Up => match self.model.focussed_panel {
                        Panel::TaskList => match self.model.selected_task {
                            Some(index) => Ok(Some(Action::SelectTask {
                                // TODO check for out of bounds
                                task_index: index + 1,
                            })),
                            None => Ok(Some(Action::SelectTask { task_index: 0 })),
                        },
                        Panel::WorkspaceList => match self.model.selected_workspace {
                            Some(index) => Ok(Some(Action::SelectWorkspace {
                                // TODO chec for out of bounds
                                workspace_index: index + 1,
                            })),
                            None => Ok(Some(Action::SelectWorkspace { workspace_index: 0 })),
                        },
                        _ => Ok(None),
                    },
                    KeyCode::Down => match self.model.focussed_panel {
                        Panel::TaskList => match self.model.selected_task {
                            Some(index) => Ok(Some(Action::SelectTask {
                                // TODO check for out of bounds
                                task_index: index - 1,
                            })),
                            None => Ok(Some(Action::SelectTask { task_index: 0 })),
                        },
                        Panel::WorkspaceList => match self.model.selected_workspace {
                            Some(index) => Ok(Some(Action::SelectWorkspace {
                                // TODO chec for out of bounds
                                workspace_index: index - 1,
                            })),
                            None => Ok(Some(Action::SelectWorkspace { workspace_index: 0 })),
                        },
                        _ => Ok(None),
                    },
                    KeyCode::Char('w') => Ok(Some(Action::FocusWorkspacePane)),
                    KeyCode::Char('l') => Ok(Some(Action::FocusTaskListPane)),
                    KeyCode::Char('t') => Ok(Some(Action::FocusTaskPane)),
                    KeyCode::Char('q') => Ok(Some(Action::Quit)),
                    _ => Ok(None),
                };
            }
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
                self.model.selected_workspace = Some(workspace_index);
                self.model.selected_task = Some(0);
            }
            Action::SelectTask { task_index } => self.model.selected_task = Some(task_index),
        }
    }
}

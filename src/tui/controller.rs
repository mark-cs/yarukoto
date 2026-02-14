use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};

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
            terminal.draw(|frame| self.view(frame))?;

            if let Some(action) = self.handle_event()? {
                self.handle_action(action);
            }
        }

        Ok(())
    }
    fn view(self: &mut Controller, frame: &mut ratatui::Frame<'_>) {
        // Nav area / main area split
        let nav_main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        view::nav_bar(frame, nav_main_layout[0], &mut self.model);

        frame.render_widget("main_view", nav_main_layout[1]);
    }
    fn handle_event(&self) -> Result<Option<Action>> {
        if event::poll(Duration::from_millis(250))? {
            let event = event::read()?;
            if let Some(key_event) = event.as_key_event() {
                return match key_event.code {
                    KeyCode::Down => match self.model.focussed_panel {
                        Panel::TaskList => match self.model.task_selection().selected() {
                            Some(index) => Ok(Some(Action::SelectTask {
                                task_index: std::cmp::max(
                                    index.saturating_add(1),
                                    self.model.task_count()?,
                                ),
                            })),
                            None => Ok(Some(Action::SelectTask { task_index: 0 })),
                        },
                        Panel::WorkspaceList => match self.model.workspace_selection().selected() {
                            Some(index) => Ok(Some(Action::SelectWorkspace {
                                workspace_index: std::cmp::max(
                                    index.saturating_add(1),
                                    self.model.workspace_count(),
                                ),
                            })),
                            None => Ok(Some(Action::SelectWorkspace { workspace_index: 0 })),
                        },
                        _ => Ok(None),
                    },
                    KeyCode::Up => match self.model.focussed_panel {
                        Panel::TaskList => match self.model.task_selection().selected() {
                            Some(index) => Ok(Some(Action::SelectTask {
                                task_index: index.saturating_sub(1),
                            })),
                            None => Ok(Some(Action::SelectTask { task_index: 0 })),
                        },
                        Panel::WorkspaceList => match self.model.workspace_selection().selected() {
                            Some(index) => Ok(Some(Action::SelectWorkspace {
                                workspace_index: index.saturating_sub(1),
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
                self.model
                    .workspace_selection_mut()
                    .select(Some(workspace_index));
                self.model.task_selection_mut().select(Some(0));
            }
            Action::SelectTask { task_index } => {
                self.model.task_selection_mut().select(Some(task_index))
            }
        }
    }
}

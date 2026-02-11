use ratatui::{DefaultTerminal, crossterm};

use crate::result::Result;
use crate::tui::model::{Model, RunningState};

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

            if crossterm::event::read()?.is_key_press() {
                self.model.running_state = RunningState::Done;
            }
        }

        Ok(())
    }
    fn view(&self, frame: &mut ratatui::Frame<'_>) {
        frame.render_widget("hello world", frame.area());
    }
}

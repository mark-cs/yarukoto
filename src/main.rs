mod result;
mod task;
mod tui;
mod workspace;

use ratatui::{self, DefaultTerminal};

use tui::model::Model;

use crate::tui::{controller::Controller, model::RunningState};

use result::Result;

fn main() -> Result<()> {
    let model = Model::default();

    let controller = Controller::new(model);

    ratatui::run(|terminal| controller.run(terminal))?;

    Ok(())
}


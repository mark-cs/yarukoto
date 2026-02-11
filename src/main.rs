mod result;
mod task;
mod tui;
mod workspace;

use ratatui::{self};

use tui::model::Model;

use tui::controller::Controller;

use result::Result;

fn main() -> Result<()> {
    let model = Model::default();

    let controller = Controller::new(model);

    ratatui::run(|terminal| controller.run(terminal))?;

    Ok(())
}

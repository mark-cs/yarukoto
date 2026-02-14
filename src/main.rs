mod result;
mod task;
mod tui;
mod workspace;

use ratatui::{self};

use tui::model::Model;

use tui::controller::Controller;

use result::Result;

use crate::workspace::Workspace;

fn main() -> Result<()> {
    let mut model = Model::default();

    model
        .workspaces_mut()
        .push(Workspace::new(&String::from("./test-workspace/"))?);

    let controller = Controller::new(model);

    ratatui::run(|terminal| controller.run(terminal))?;

    Ok(())
}

mod task;
mod workspace;

use std::io::Error;
use std::result::Result;

use clap::Parser;
use ratatui::{DefaultTerminal, Frame, crossterm};
use workspace::Workspace;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'w', long = "workspace")]
    workspace: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let workspace = Workspace::new(&args.workspace)?;
    println!("Workspace: {:?}", workspace);
    let tasks = workspace.tasks()?;
    println!("Tasks: {:?}", tasks);

    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

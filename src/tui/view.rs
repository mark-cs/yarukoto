use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, List, ListDirection, Widget},
};

use crate::{task::Task, tui::model::Model, workspace::Workspace};

pub fn nav_bar(frame: &mut ratatui::Frame<'_>, container: Rect, model: &mut Model) {
    let nav_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(container);
    frame.render_stateful_widget(
        workspace_list(&model.workspaces()),
        nav_layout[0],
        model.workspace_selection_mut(),
    );

    frame.render_widget(
        task_list(&model.tasks().unwrap_or(Vec::new())),
        nav_layout[1],
    );
}

pub fn workspace_list<'a>(workspaces: &[Workspace]) -> List<'a> {
    List::new(workspaces.iter().map(|w| w.name.to_owned()))
        .block(Block::bordered().title("Workspaces"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}
pub fn task_list<'a>(tasks: &[Task]) -> List<'a> {
    List::new(tasks.iter().map(|w| w.title.to_owned()))
        .block(Block::bordered().title("Tasks"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

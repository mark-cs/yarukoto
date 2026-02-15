use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, List, ListDirection, Paragraph},
};

use crate::{
    task::Task,
    tui::model::{Model, Panel},
    workspace::Workspace,
};

pub fn view(frame: &mut ratatui::Frame<'_>, model: &mut Model) {
    // Nav area / main area split
    let nav_main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());
    nav_bar(frame, nav_main_layout[0], model);
    if let Some(index) = model.task_selection().selected() {
        frame.render_widget(
            task(&model.tasks()[index], model.focussed_panel == Panel::Task),
            nav_main_layout[1],
        )
    };
}

fn task<'a>(task: &Task, panel_selected: bool) -> Paragraph<'a> {
    Paragraph::new(task.title.clone()).block(
        Block::bordered()
            .border_type(if panel_selected {
                BorderType::Double
            } else {
                BorderType::Plain
            })
            .title("Task (t)"),
    )
}

fn nav_bar(frame: &mut ratatui::Frame<'_>, container: Rect, model: &mut Model) {
    let nav_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(container);
    frame.render_stateful_widget(
        workspace_list(
            model.workspaces(),
            model.focussed_panel == Panel::WorkspaceList,
        ),
        nav_layout[0],
        model.workspace_selection_mut(),
    );

    frame.render_stateful_widget(
        task_list(model.tasks(), model.focussed_panel == Panel::TaskList),
        nav_layout[1],
        model.task_selection_mut(),
    );
}

fn workspace_list<'a>(workspaces: &[Workspace], panel_selected: bool) -> List<'a> {
    List::new(workspaces.iter().map(|w| w.name.to_owned()))
        .block(
            Block::bordered()
                .border_type(if panel_selected {
                    BorderType::Double
                } else {
                    BorderType::Plain
                })
                .title("Workspaces (w)"),
        )
        .style(Style::new().white())
        .highlight_style(Style::new().blue().italic())
        .highlight_symbol("▶")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

fn task_list<'a>(tasks: &[Task], panel_selected: bool) -> List<'a> {
    List::new(tasks.iter().map(|w| w.title.to_owned()))
        .block(
            Block::bordered()
                .border_type(if panel_selected {
                    BorderType::Double
                } else {
                    BorderType::Plain
                })
                .title("Tasks (l)"),
        )
        .style(Style::new().white())
        .highlight_style(Style::new().blue().italic())
        .highlight_symbol("▶")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

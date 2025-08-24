use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
};

use crate::tui::app::{App, DialogState, Tab};

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Render header with tabs
    render_header(f, chunks[0], app);

    // Render main content based on current tab
    match app.current_tab {
        Tab::Tracked => render_tracked_dotfiles(f, chunks[1], app),
        Tab::Staged => render_staged_dotfiles(f, chunks[1], app),
        Tab::Help => render_help(f, chunks[1]),
    }

    // Render footer
    render_footer(f, chunks[2], app);

    // Render dialogs if any
    if app.dialog_state != DialogState::None {
        render_dialog(f, app);
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tab_titles = vec!["Tracked", "Staged", "Help"];
    let selected_tab = match app.current_tab {
        Tab::Tracked => 0,
        Tab::Staged => 1,
        Tab::Help => 2,
    };

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Dotzilla TUI"))
        .select(selected_tab)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Blue),
        );

    f.render_widget(tabs, area);
}

fn render_tracked_dotfiles(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .tracked_dotfiles
        .iter()
        .enumerate()
        .map(|(i, (dot_path, entry))| {
            let style = if i == app.selected_tracked {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let status_color = match entry.status {
                crate::models::DotfileStatus::Tracked => Color::Green,
                crate::models::DotfileStatus::Staged => Color::Yellow,
                crate::models::DotfileStatus::Untracked => Color::Gray,
                crate::models::DotfileStatus::Modified => Color::Red,
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("{:?}", entry.status),
                    Style::default().fg(status_color),
                ),
                Span::raw(" "),
                Span::styled(dot_path.to_string(), style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tracked Dotfiles")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(list, area);
}

fn render_staged_dotfiles(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .staged_dotfiles
        .iter()
        .enumerate()
        .map(|(i, (dot_path, entry))| {
            let style = if i == app.selected_staged {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let status_color = match entry.status {
                crate::models::DotfileStatus::Tracked => Color::Green,
                crate::models::DotfileStatus::Staged => Color::Yellow,
                crate::models::DotfileStatus::Untracked => Color::Gray,
                crate::models::DotfileStatus::Modified => Color::Red,
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("{:?}", entry.status),
                    Style::default().fg(status_color),
                ),
                Span::raw(" "),
                Span::styled(dot_path.to_string(), style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Staged Dotfiles")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(list, area);
}

fn render_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from("Dotzilla TUI - Keyboard Shortcuts"),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  ↑/k        - Move up"),
        Line::from("  ↓/j        - Move down"),
        Line::from("  Tab        - Next tab"),
        Line::from("  Shift+Tab  - Previous tab"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  a          - Add dotfile"),
        Line::from("  d          - Remove/Delete selected"),
        Line::from("  s          - Stage/Unstage selected"),
        Line::from("  l          - Link all staged"),
        Line::from("  u          - Unlink all"),
        Line::from("  c          - Commit staged"),
        Line::from("  r          - Refresh"),
        Line::from("  q          - Quit"),
        Line::from(""),
        Line::from("Dialog:"),
        Line::from("  Enter      - Confirm"),
        Line::from("  Esc        - Cancel"),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .title_alignment(Alignment::Center),
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let mut text = vec![
        Span::raw("Press "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to quit, "),
        Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to switch tabs"),
    ];

    // Show messages
    if let Some(ref message) = app.message {
        text = vec![Span::styled(
            format!("✓ {}", message),
            Style::default().fg(Color::Green),
        )];
    } else if let Some(ref error) = app.error_message {
        text = vec![Span::styled(
            format!("✗ {}", error),
            Style::default().fg(Color::Red),
        )];
    }

    let footer = Paragraph::new(Line::from(text))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}

fn render_dialog(f: &mut Frame, app: &App) {
    let size = f.area();

    match &app.dialog_state {
        DialogState::AddFile => {
            let area = centered_rect(50, 20, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .title("Add Dotfile")
                .title_alignment(Alignment::Center);

            let inner = block.inner(area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .split(inner);

            let instruction =
                Paragraph::new("Enter the path to the dotfile:").alignment(Alignment::Center);
            f.render_widget(instruction, chunks[0]);

            let input = Paragraph::new(app.input_text.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(input, chunks[2]);

            let help = Paragraph::new("Press Enter to confirm, Esc to cancel")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));
            f.render_widget(help, chunks[3]);
        }
        DialogState::Confirm(action) => {
            let area = centered_rect(40, 15, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .title("Confirm")
                .title_alignment(Alignment::Center);

            let inner = block.inner(area);
            f.render_widget(block, area);

            let text = match action.as_str() {
                "remove" => "Are you sure you want to remove this dotfile?",
                _ => "Are you sure?",
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Length(1)])
                .split(inner);

            let message = Paragraph::new(text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(message, chunks[0]);

            let help = Paragraph::new("Press Enter to confirm, Esc to cancel")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));
            f.render_widget(help, chunks[1]);
        }
        DialogState::None => {}
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

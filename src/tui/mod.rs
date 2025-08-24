use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

mod app;
mod ui;

use app::{App, AppResult, DialogState};

pub fn run(repo_path: String) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new(repo_path)?;
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> AppResult<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') if app.dialog_state == DialogState::None => return Ok(()),
                    KeyCode::Char('r') if app.dialog_state == DialogState::None => app.refresh()?,
                    KeyCode::Char('a') if app.dialog_state == DialogState::None => {
                        app.show_add_dialog()
                    }
                    KeyCode::Char('d') if app.dialog_state == DialogState::None => {
                        app.remove_selected()?
                    }
                    KeyCode::Char('s') if app.dialog_state == DialogState::None => {
                        app.toggle_stage_selected()?
                    }
                    KeyCode::Char('l') if app.dialog_state == DialogState::None => {
                        app.link_staged()?
                    }
                    KeyCode::Char('u') if app.dialog_state == DialogState::None => {
                        app.unlink_all()?
                    }
                    KeyCode::Char('c') if app.dialog_state == DialogState::None => {
                        app.commit_staged()?
                    }
                    KeyCode::Up | KeyCode::Char('k') if app.dialog_state == DialogState::None => {
                        app.previous()
                    }
                    KeyCode::Down | KeyCode::Char('j') if app.dialog_state == DialogState::None => {
                        app.next()
                    }
                    KeyCode::Tab if app.dialog_state == DialogState::None => app.next_tab(),
                    KeyCode::BackTab if app.dialog_state == DialogState::None => app.previous_tab(),
                    KeyCode::Enter => app.handle_enter()?,
                    KeyCode::Esc => app.handle_escape(),
                    KeyCode::Backspace => app.handle_backspace(),
                    KeyCode::Char(c) => app.handle_char_input(c),
                    _ => {}
                }
            }
        }
    }
}

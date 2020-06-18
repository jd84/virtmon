mod ui;
mod util;

use crate::util::event::{Event, Events};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use harvest::system::{RemoteSystem, SystemData};
use harvest::SystemData as LocalSystem;
use std::io::{self, Write};
use std::panic;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_panic_hook();
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let events = Events::new();
    let mut sys_data = LocalSystem::default();
    let mut r_sys_data = RemoteSystem::new().await;

    loop {
        terminal.draw(|mut f| {
            let all_cpus = r_sys_data.get_cpus();

            // setup base layout
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length((all_cpus.len() + 2) as u16),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // draw all widgets
            ui::draw_cpus(&mut f, &all_cpus, chunks[0]);
            ui::draw_processes(&mut f, &sys_data.get_processes(), chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => {
                sys_data.refresh();
                r_sys_data.refresh().await;
            }
        }
    }

    Ok(())
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let mut stdout = io::stdout();
        let msg = match panic_info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match panic_info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", backtrace::Backtrace::new());
        disable_raw_mode().unwrap();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture).unwrap();
        execute!(
            stdout,
            Print(format!(
                "thread '<unnamed>' paniced at '{}', {}\n\r{}",
                msg,
                panic_info.location().unwrap(),
                stacktrace
            ))
        )
        .unwrap();
    }));
}

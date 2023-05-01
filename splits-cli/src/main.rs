mod app;
mod file;
mod style;
mod ui;

use std::{
    io,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::app::App;

#[derive(Parser)]
struct Args {
    splits_file: PathBuf,
    #[clap(long, short, default_value_t = 10)]
    tick_rate: i32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let app = match args.splits_file.try_exists()? {
    //     true => App::from_file(args.splits_file),
    //     false => App::new(Run::, splits_file)
    // }
    let app = App::from_file(args.splits_file)?;
    let res = run_app(
        &mut terminal,
        app,
        Duration::from_secs_f64(1.0 / args.tick_rate as f64),
    );

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    res
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> anyhow::Result<()> {
    let last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    #[allow(clippy::single_match)]
                    match key.code {
                        KeyCode::Char(c) => app.on_key(c)?,
                        _ => {}
                    }
                }
            }
        }

        if app.should_exit {
            break;
        }
    }
    Ok(())
}

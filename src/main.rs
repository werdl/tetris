// filepath: /tetris/tetris/src/main.rs
use std::io::{self, Write};
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::{event, terminal, ExecutableCommand};

mod grid;
mod config;
mod shape;
mod block;
mod ui;
mod utils;
mod constants {
    pub const ROW_WIDTH: usize = 10;
    pub const ROW_HEIGHT: usize = 20;

    pub const I: &str = "██\n██\n██\n██";
    pub const O: &str = "████\n████";
    pub const T: &str = "██████\n  ██";
    pub const S: &str = "  ████\n████";
    pub const Z: &str = "████\n  ████";
    pub const L: &str = "██████\n██";
    pub const J: &str = "██████\n    ██";
}


use grid::Grid;
use utils::{handle_events, end_game};

fn run(terminal: &mut ratatui::DefaultTerminal, opts: Options) -> Result<(), String> {
    let mut grid = Grid::new();
    let mut last_update = Instant::now();

    if opts.level.unwrap_or(1) > 10 || opts.level.unwrap_or(1) == 0 {
        panic!("0 < level <= 10 not met");
    }

    grid.level = opts.level.unwrap_or(1);

    let mut update_interval = Duration::from_millis((500.0 * ((11.0 - (grid.level as f32)) / 10.0)).ceil() as u64);

    grid.next(None);

    loop {
        terminal
            .draw(|frame| ui::draw(frame, grid.clone()))
            .map_err(|e| e.to_string())?;

        if last_update.elapsed() >= update_interval && !grid.paused {
            grid.bring_down(None, |g| {
                end_game(&g);
            });

            last_update = Instant::now();
            update_interval = Duration::from_millis((500.0 * ((11.0 - (grid.level as f32)) / 10.0)).ceil() as u64);
        }

        while event::poll(Duration::from_millis(1)).unwrap() {
            if handle_events(&mut grid, |g| {
                end_game(&g);
            })? {
                cleanup_terminal();
                std::process::exit(0);
            }
        }
    }
}

fn cleanup_terminal() {
    terminal::disable_raw_mode().unwrap();
    io::stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    io::stdout().execute(crossterm::event::DisableMouseCapture).unwrap();
    io::stdout().execute(crossterm::cursor::Show).unwrap();
    io::stdout().execute(crossterm::style::SetForegroundColor(crossterm::style::Color::Reset)).unwrap();
    io::stdout().flush().unwrap();
}

#[derive(Parser)]
struct Options {
    #[arg(short, long)]
    level: Option<u32>,
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode().unwrap();
    io::stdout().execute(terminal::EnterAlternateScreen).unwrap();
    io::stdout().execute(crossterm::event::EnableMouseCapture).unwrap();
    let mut terminal = ratatui::init();

    let opts = Options::parse();

    let out = run(&mut terminal, opts);
    if out.is_err() {
        panic!("Error: {}", out.unwrap_err());
    }

    Ok(())
}
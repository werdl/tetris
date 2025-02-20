// filepath: /tetris/tetris/src/main.rs

use std::io::{self, Write};
use std::time::{Duration, Instant};

use clap::Parser;
use config::Config;
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

fn run(terminal: &mut ratatui::DefaultTerminal, opts: Options, cfg: Config) -> Result<(), String> {
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
            }, cfg.clone(), terminal)? {
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
    /// Start at a specific level (1-10)
    #[arg(short, long)]
    level: Option<u32>,

    /// Path to the config file
    #[arg(short, long, default_value = "config.json")]
    config: String,

    /// Create a configuration file
    #[command(subcommand)]
    create: Option<CreateConfig>,
}

#[derive(Parser)]
enum CreateConfig {
    /// Create a new configuration file
    New,
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode().unwrap();
    let opts = Options::parse();

    // check if we need to create a new config file
    let config = match opts.create {
        Some(CreateConfig::New) => {
            let cfg = config::interactive_config();
            let serialized = serde_json::to_string(&cfg).unwrap();
            std::fs::write("config.json", serialized).unwrap();
            cfg
        }
        None => {
            let file = std::fs::read_to_string(&opts.config);
            
            match file {
                Ok(file) => serde_json::from_str(&file).unwrap(),
                Err(_) => {
                    let cfg = config::interactive_config();
                    let serialized = serde_json::to_string(&cfg).unwrap();
                    std::fs::write("config.json", serialized).unwrap();
                    cfg
                }
            }
        }
    };
    io::stdout().execute(terminal::EnterAlternateScreen).unwrap();
    io::stdout().execute(crossterm::event::EnableMouseCapture).unwrap();
    let mut terminal = ratatui::init();



    let out = run(&mut terminal, opts, config);
    if out.is_err() {
        panic!("Error: {}", out.unwrap_err());
    }

    Ok(())
}
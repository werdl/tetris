// This file contains utility functions for the Tetris game.

use std::io::{self, Stdout};
use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, ExecutableCommand};
use ratatui::{prelude::CrosstermBackend, Terminal};
use crate::{cleanup_terminal, config::Config, grid::Grid, ui};

pub fn handle_events(grid: &mut Grid, mut end_cb: impl FnMut(Grid) -> (), config: Config, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<bool, String> {
    match event::read() {
        Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
            val if val == config.hard_drop.code => {
                grid.paused = false;
                while grid.bring_down(None, &mut end_cb) {}
            },
            val if val == config.soft_drop.code => {
                grid.paused = false;
                while grid.bring_down(None, &mut end_cb) {
                    std::thread::sleep(std::time::Duration::from_millis(config.soft_drop_ms_per_cell as u64));
                    // redraw the screen
                    terminal.draw(|frame| ui::draw(frame, grid.clone())).unwrap();

                    // and also check for events so that fancy sliding can happen
                    if event::poll(std::time::Duration::from_millis(0)).unwrap() {
                        if let Ok(Event::Key(key)) = event::read() {
                            if key.kind == KeyEventKind::Press {
                                handle_key_event(grid, config.clone(), terminal, key.code);
                            }
                        }
                    }
                }
            },
            _ => {
                handle_key_event(grid, config.clone(), terminal, key.code);
            }

        },

        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }
    grid.remove_full_rows();
    Ok(false)
}

fn handle_key_event(grid: &mut Grid, config: Config, terminal: &mut Terminal<CrosstermBackend<Stdout>>, keycode: KeyCode) {
    match keycode {
            val if val == config.move_left.code => {
                grid.move_active_blocks(-1, 0);
            },
            val if val == config.move_right.code => {
                grid.move_active_blocks(1, 0);
            },
            val if val == config.rotate_cw.code => {
                grid.rotate_active_block();
            },
            val if val == config.rotate_ccw.code => {
                grid.rotate_active_block();
                grid.rotate_active_block();
                grid.rotate_active_block();
            },
            val if val == config.hold.code => {
                grid.hold();
            },
            val if val == config.pause.code => {
                grid.paused = !grid.paused;
            },
            val if val == config.quit.code => {
                end_game(grid);
            },
            _ => {}
    }
}

pub fn end_game(grid: &Grid) {
    // clear the screen
    io::stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    cleanup_terminal();
    println!("Game Over!");
    println!("Score: {}", grid.score);
    println!("Level: {}", grid.level);
    println!("Press any key to exit...");

    // now wait for the user to press a key
    loop {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                break;
            }
        }
    }

    std::process::exit(0);
}
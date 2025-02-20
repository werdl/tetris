// This file contains utility functions for the Tetris game.

use std::io;
use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, ExecutableCommand};
use crate::{cleanup_terminal, grid::Grid};

pub fn handle_events(grid: &mut Grid, mut end_cb: impl FnMut(Grid) -> ()) -> Result<bool, String> {
    match event::read() {
        Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Up => {
                grid.paused = false;
                grid.rotate_active_block();
            }
            KeyCode::Down => {
                grid.paused = false;
                while grid.bring_down(None, &mut end_cb) {
                }
            }
            KeyCode::Left => {
                grid.paused = false;
                grid.move_active_blocks(-1, 0);
            }
            KeyCode::Right => {
                grid.paused = false;
                grid.move_active_blocks(1, 0);
            }
            KeyCode::Char('p') => {
                grid.paused = !grid.paused;
            }
            KeyCode::Char('h') => {
                grid.paused = false;
                grid.hold();
            }
            _ => {
                grid.paused = false;
            }
        },

        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }
    grid.remove_full_rows();
    Ok(false)
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
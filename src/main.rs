use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use clap::Parser;
use colored::{ColoredString, Colorize};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal, ExecutableCommand,
};
use ratatui::{
    style::{Color, Stylize},
    symbols::block,
    text::{Line, Text},
    widgets::Paragraph,
    Frame,
};

const ROW_WIDTH: usize = 10;
const ROW_HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Full(Shape),
    Empty,
    Active(Shape),
}

#[derive(Clone, Copy)]
struct Row {
    cells: [Block; ROW_WIDTH],
}

#[derive(Clone)]
struct Grid {
    rows: [Row; ROW_HEIGHT],
    active_shape: Option<Shape>,
    next_shape: Shape,
    shape_history: Vec<Shape>,
    score: u32,
    level: u32,
    shape_has_existed_for: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Shape {
    fn random() -> Shape {
        use rand::Rng;
        match rand::rng().random_range(0..7) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            _ => Shape::L,
        }
    }
}

impl Grid {
    fn new() -> Self {
        Grid {
            rows: [Row {
                cells: [Block::Empty; ROW_WIDTH],
            }; ROW_HEIGHT],
            active_shape: None,
            next_shape: Shape::random(),
            shape_history: Vec::new(),
            score: 0,
            level: 1,
            shape_has_existed_for: 0,
        }
    }

    fn next(&mut self) {
        if self.shape_has_existed_for != 0 {
            self.score += (((ROW_HEIGHT as u32) - self.shape_has_existed_for) as f32 * ((self.level as f32)/2.0).ceil()).ceil() as u32;
        }
        self.shape_has_existed_for = 0;
        self.shape_history.push(self.next_shape);
        if self.shape_history.len() % 25 == 0 {
            self.level += 1;
        }
        self.active_shape = Some(self.next_shape);
        self.next_shape = Shape::random();
        self.spawn(self.active_shape.unwrap()).unwrap();
    }

    fn set(&mut self, (x, y): (usize, usize), block: Block) {
        self.rows[y].cells[x] = block;
    }

    fn bring_down(&mut self) -> bool {
        self.shape_has_existed_for += 1;
        let mut active_blocks = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.cells.iter().enumerate() {
                if let Block::Active(shape) = *cell {
                    active_blocks.push((x, y, shape));
                }
            }
        }

        let mut new_active_blocks = Vec::new();

        for (x, y, shape) in active_blocks.iter() {
            new_active_blocks.push((*x, *y + 1, *shape));
        }

        for (x, y, _) in new_active_blocks.iter() {
            if *y == &ROW_HEIGHT - 1 || matches!(self.rows[y + 1].cells[*x], Block::Full(_)) {
                for (x, y, _shape) in active_blocks.iter() {
                    self.rows[*y].cells[*x] = Block::Empty;
                }
                for (x, y, shape) in new_active_blocks.iter() {
                    self.rows[*y].cells[*x] = Block::Full(*shape);
                }
                self.next();
                return false;
            }
        }

        if new_active_blocks
            .iter()
            .any(|(_, y, _)| *y == ROW_HEIGHT - 1)
        {
            for (x, y, _shape) in active_blocks.iter() {
                self.rows[*y].cells[*x] = Block::Empty;
            }
            for (x, y, shape) in new_active_blocks.iter() {
                self.rows[*y].cells[*x] = Block::Full(*shape);
            }
            self.next();
            return false;
        }

        for (x, y, _shape) in active_blocks.iter() {
            self.rows[*y].cells[*x] = Block::Empty;
        }

        for (x, y, shape) in new_active_blocks.iter() {
            self.rows[*y].cells[*x] = Block::Active(*shape);
        }

        true
    }

    fn move_active_blocks(&mut self, dx: i32, dy: i32) {
        let mut active_blocks = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.cells.iter().enumerate() {
                if let Block::Active(shape) = *cell {
                    active_blocks.push((x, y, shape));
                }
            }
        }

        let mut new_active_blocks = Vec::new();

        for (x, y, shape) in active_blocks.iter() {
            new_active_blocks.push((
                x.wrapping_add(dx as usize),
                y.wrapping_add(dy as usize),
                *shape,
            ));
        }

        for (x, y, _) in new_active_blocks.iter() {
            if *x >= ROW_WIDTH
                || *y >= ROW_HEIGHT
                || matches!(self.rows[*y].cells[*x], Block::Full(_))
            {
                return;
            }
        }

        for (x, y, _shape) in active_blocks.iter() {
            self.rows[*y].cells[*x] = Block::Empty;
        }

        for (x, y, shape) in new_active_blocks.iter() {
            self.rows[*y].cells[*x] = Block::Active(*shape);
        }
    }

    fn rotate_active_block(&mut self) {
        let mut active_blocks = Vec::new();

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.cells.iter().enumerate() {
                if let Block::Active(shape) = *cell {
                    active_blocks.push((x, y, shape));
                }
            }
        }

        let shape = match self.active_shape {
            Some(shape) => shape,
            None => return,
        };

        let centre = match shape {
            Shape::I => (active_blocks[1].0, active_blocks[1].1),
            Shape::O => return,
            _ => {
                let min_x = active_blocks.iter().map(|(x, _, _)| x).min().unwrap();
                let max_x = active_blocks.iter().map(|(x, _, _)| x).max().unwrap();
                let min_y = active_blocks.iter().map(|(_, y, _)| y).min().unwrap();
                let max_y = active_blocks.iter().map(|(_, y, _)| y).max().unwrap();

                ((min_x + max_x) / 2, (min_y + max_y) / 2)
            }
        };

        let rotation_matrix = [[0, 1], [-1, 0]];

        let mut new_active_blocks = Vec::new();

        for (x, y, shape) in active_blocks.clone() {
            let x = x as i32 - centre.0 as i32;
            let y = y as i32 - centre.1 as i32;
            let new_x = x * rotation_matrix[0][0] + y * rotation_matrix[0][1];
            let new_y = x * rotation_matrix[1][0] + y * rotation_matrix[1][1];

            let new_x = new_x + centre.0 as i32;
            let new_y = new_y + centre.1 as i32;

            if new_x < 0 || new_x >= ROW_WIDTH as i32 || new_y < 0 || new_y >= ROW_HEIGHT as i32 {
                return;
            }

            new_active_blocks.push((new_x as usize, new_y as usize, shape));
        }
        let max_y = new_active_blocks.iter().map(|&(_, y, _)| y).max().unwrap();
        if max_y > centre.1 {
            let offset = max_y - centre.1;
            for (_, y, _) in new_active_blocks.iter_mut() {
                *y += offset;
            }
        }

        let max_x = new_active_blocks.iter().map(|&(x, _, _)| x).max().unwrap();
        if max_x >= ROW_WIDTH {
            let offset = max_x - ROW_WIDTH + 1;
            for (x, _, _) in new_active_blocks.iter_mut() {
                *x -= offset;
            }
        }

        // now make

        for (x, y, _shape) in active_blocks {
            self.rows[y].cells[x] = Block::Empty;
        }

        for (x, y, shape) in new_active_blocks {
            self.rows[y].cells[x] = Block::Active(shape);
        }
    }

    fn remove_full_rows(&mut self) {
        let mut full_rows = Vec::new();

        for (y, row) in self.rows.iter().enumerate() {
            if row.cells.iter().all(|cell| matches!(cell, Block::Full(_))) {
                full_rows.push(y);
            }
        }

        for row in full_rows.iter() {
            for y in (0..*row).rev() {
                for x in 0..ROW_WIDTH {
                    self.rows[y + 1].cells[x] = self.rows[y].cells[x];
                }
            }
        }
    }

    fn spawn(&mut self, shape: Shape) -> Result<(), String> {
        self.remove_full_rows();
        match shape {
            Shape::I => {
                let x = ROW_WIDTH / 2;
                let y = 0;
                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y + 1].cells[x] == Block::Empty
                    && self.rows[y + 2].cells[x] == Block::Empty
                    && self.rows[y + 3].cells[x] == Block::Empty
                {
                    self.set((x, y), Block::Active(shape));
                    self.set((x, y + 1), Block::Active(shape));
                    self.set((x, y + 2), Block::Active(shape));
                    self.set((x, y + 3), Block::Active(shape));
                } else {
                    return Err("Cannot spawn shape".to_string());
                }
            }

            Shape::O => {
                let x = ROW_WIDTH / 2;
                let y = 0;

                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y].cells[x + 1] == Block::Empty
                    && self.rows[y + 1].cells[x] == Block::Empty
                    && self.rows[y + 1].cells[x + 1] == Block::Empty
                {
                    self.set((x, y), Block::Active(shape));
                    self.set((x + 1, y), Block::Active(shape));
                    self.set((x, y + 1), Block::Active(shape));
                    self.set((x + 1, y + 1), Block::Active(shape));
                } else {
                    return Err("Cannot spawn shape".to_string());
                }
            }

            Shape::T => {
                let x = ROW_WIDTH / 2;
                let y = 0;

                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y].cells[x - 1] == Block::Empty
                    && self.rows[y].cells[x + 1] == Block::Empty
                    && self.rows[y + 1].cells[x] == Block::Empty
                {
                    self.set((x, y), Block::Active(shape));
                    self.set((x - 1, y), Block::Active(shape));
                    self.set((x + 1, y), Block::Active(shape));
                    self.set((x, y + 1), Block::Active(shape));
                } else {
                    return Err("Cannot spawn shape".to_string());
                }
            }

            Shape::S => {
                let x = ROW_WIDTH / 2;
                let y = 0;

                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y].cells[x + 1] == Block::Empty
                    && self.rows[y + 1].cells[x] == Block::Empty
                    && self.rows[y + 1].cells[x - 1] == Block::Empty
                {
                    self.set((x, y), Block::Active(shape));
                    self.set((x + 1, y), Block::Active(shape));
                    self.set((x, y + 1), Block::Active(shape));
                    self.set((x - 1, y + 1), Block::Active(shape));
                } else {
                    return Err("Cannot spawn shape".to_string());
                }
            }

            Shape::Z => {
                let x = ROW_WIDTH / 2;
                let y = 0;

                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y].cells[x - 1] == Block::Empty
                    && self.rows[y + 1].cells[x] == Block::Empty
                    && self.rows[y + 1].cells[x + 1] == Block::Empty
                {
                    self.set((x, y), Block::Active(shape));
                    self.set((x - 1, y), Block::Active(shape));
                    self.set((x, y + 1), Block::Active(shape));
                    self.set((x + 1, y + 1), Block::Active(shape));
                } else {
                    return Err("Cannot spawn shape".to_string());
                }
            }

            Shape::J => {
                let x = ROW_WIDTH / 2;
                let y = 0;

                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y].cells[x - 1] == Block::Empty
                    && self.rows[y].cells[x + 1] == Block::Empty
                    && self.rows[y + 1].cells[x + 1] == Block::Empty
                {
                    self.set((x, y), Block::Active(shape));
                    self.set((x - 1, y), Block::Active(shape));
                    self.set((x + 1, y), Block::Active(shape));
                    self.set((x + 1, y + 1), Block::Active(shape));
                } else {
                    return Err("Cannot spawn shape".to_string());
                }
            }

            Shape::L => {
                let x = ROW_WIDTH / 2;
                let y = 0;

                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y].cells[x - 1] == Block::Empty
                    && self.rows[y].cells[x + 1] == Block::Empty
                    && self.rows[y + 1].cells[x - 1] == Block::Empty
                {
                    self.set((x, y), Block::Active(shape));
                    self.set((x - 1, y), Block::Active(shape));
                    self.set((x + 1, y), Block::Active(shape));
                    self.set((x - 1, y + 1), Block::Active(shape));
                } else {
                    return Err("Cannot spawn shape".to_string());
                }
            }
        }

        self.active_shape = Some(shape);
        self.bring_down();

        Ok(())
    }
}

fn handle_events(grid: &mut Grid) -> Result<bool, String> {
    match event::read() {
        Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Up => {
                grid.rotate_active_block();
            }
            KeyCode::Down => {
                // bring shape to the bottom
                while grid.bring_down() {}
            }
            KeyCode::Left => {
                grid.move_active_blocks(-1, 0);
            }
            KeyCode::Right => {
                grid.move_active_blocks(1, 0);
            }
            _ => {}
        },

        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }
    grid.remove_full_rows();
    Ok(false)
}

fn run(terminal: &mut ratatui::DefaultTerminal, opts: Options) -> Result<(), String> {
    let mut grid = Grid::new();
    let mut last_update = Instant::now();

    if opts.level.unwrap_or(1) > 10 || opts.level.unwrap_or(1) == 0 {
        panic!("0 < level <= 10 not met");
    }

    grid.level = opts.level.unwrap_or(1);


    let mut update_interval = Duration::from_millis((500.0 * ((11.0-(grid.level as f32))/10.0)).ceil() as u64);

    grid.next();

    loop {
        terminal
            .draw(|frame| draw(frame, grid.clone()))
            .map_err(|e| e.to_string())?;

        if last_update.elapsed() >= update_interval {
            grid.bring_down();
            last_update = Instant::now();
            update_interval = Duration::from_millis((500.0 * ((11.0-(grid.level as f32))/10.0)).ceil() as u64);
        }

        while event::poll(Duration::from_millis(10)).unwrap() {
            if handle_events(&mut grid)? {
                terminal
                    .backend_mut()
                    .execute(terminal::LeaveAlternateScreen)
                    .unwrap();
                terminal::disable_raw_mode().unwrap();
                io::stdout()
                    .execute(terminal::LeaveAlternateScreen)
                    .unwrap();
                io::stdout()
                    .execute(crossterm::event::DisableMouseCapture)
                    .unwrap();
                io::stdout().flush().unwrap();
                std::process::exit(0);
            }
        }
    }
}

fn get_cell_repr(cell: &Block) -> String {
    match cell {
        Block::Full(_) => format!("{}{}", block::FULL, block::FULL),
        Block::Empty => ". ".to_string(),
        Block::Active(_) => format!("{}{}", block::FULL, block::FULL),
    }
}

fn draw(frame: &mut Frame, grid: Grid) {
    let lines = grid
        .rows
        .iter()
        .map(|row| {
            row.cells
                .iter()
                .map(|cell| {
                    let cell_repr = get_cell_repr(cell);
                    match cell {
                        Block::Active(shape) | Block::Full(shape) => match shape {
                            Shape::I => cell_repr.fg(Color::Red),
                            Shape::O => cell_repr.fg(Color::Blue),
                            Shape::T => cell_repr.fg(Color::Rgb(255, 165, 0)),
                            Shape::S => cell_repr.fg(Color::Green),
                            Shape::Z => cell_repr.fg(Color::Cyan),
                            Shape::J => cell_repr.fg(Color::White),
                            Shape::L => cell_repr.fg(Color::Magenta),
                        },
                        _ => cell_repr.fg(Color::White),
                    }
                })
                .collect::<Line>()
        })
        .collect::<Vec<_>>();

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text);

    let area = frame.area();
    let centered_area = ratatui::layout::Rect::new(
        area.x + (area.width - ROW_WIDTH as u16 * 2) / 2,
        area.y + (area.height - ROW_HEIGHT as u16) / 2,
        ROW_WIDTH as u16 * 2,
        ROW_HEIGHT as u16,
    );

    let outline_area = ratatui::layout::Rect::new(
        centered_area.x - 1,
        centered_area.y - 1,
        centered_area.width + 2,
        centered_area.height + 2,
    );

    frame.render_widget(
        ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL),
        outline_area,
    );
    frame.render_widget(paragraph, centered_area);

    // Help paragraph
    let help_text = Text::from(vec![
        Line::from("Controls:"),
        Line::from("  Up: Rotate"),
        Line::from("  Down: Drop"),
        Line::from("  Left: Move Left"),
        Line::from("  Right: Move Right"),
        Line::from("  Q: Quit"),
    ]);
    let help_paragraph = Paragraph::new(help_text);
    let help_area = ratatui::layout::Rect::new(
        area.x + 1,
        area.y + 1,
        20,
        7,
    );
    frame.render_widget(help_paragraph, help_area);

    // Info paragraph
    let info_text = Text::from(vec![
        Line::from("Tetris Game"),
        Line::from(format!("Score: {}", grid.score)),
        Line::from(format!("Level: {}", grid.level)),
    ]);
    let info_paragraph = Paragraph::new(info_text);
    let info_area = ratatui::layout::Rect::new(
        area.x + area.width - 21,
        area.y + 1,
        20,
        7,
    );
    frame.render_widget(info_paragraph, info_area);
}

#[derive(Parser)]
struct Options {
    #[arg(short, long)]
    level: Option<u32>,
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let opts = Options::parse();

    let out = run(&mut terminal, opts);
    if out.is_err() {
        panic!("Error: {}", out.unwrap_err());
    }

    Ok(())
}

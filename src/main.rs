use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal, ExecutableCommand,
};
use ratatui::{symbols::block, widgets::Paragraph, Frame};

const ROW_WIDTH: usize = 10;
const ROW_HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Full,
    Empty,
    Active,
}

#[derive(Clone, Copy)]
struct Row {
    cells: [Block; ROW_WIDTH],
}

#[derive(Clone, Copy)]
struct Grid {
    rows: [Row; ROW_HEIGHT],
    active_shape: Option<Shape>,
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Grid {
    fn new() -> Self {
        Grid {
            rows: [Row {
                cells: [Block::Empty; ROW_WIDTH],
            }; ROW_HEIGHT],
            active_shape: None,
        }
    }

    fn set(&mut self, (x, y): (usize, usize), block: Block) {
        self.rows[y].cells[x] = block;
    }

    fn bring_down(&mut self) {
        let mut new_rows = [Row { cells: [Block::Empty; ROW_WIDTH] }; ROW_HEIGHT];
        for y in 0..ROW_HEIGHT - 1 {
            for x in 0..ROW_WIDTH {
                new_rows[y + 1].cells[x] = self.rows[y].cells[x];
            }
        }
        self.rows = new_rows;
    }

    fn rotate_active_block(&mut self) {
        // first, detect the type of shape
        let mut active_blocks = Vec::new();

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.cells.iter().enumerate() {
                if *cell == Block::Active {
                    active_blocks.push((x, y));
                }
            }
        }

        let shape = match self.active_shape {
            Some(shape) => shape,
            None => return,
        };

        // find the centre of the shape (different for each shape, hardcoded)
        let centre = match shape {
            Shape::I => active_blocks[1],
            Shape::O => return,
            _ => {
                // with any other shapes, pick the block in the centre of a 3x3 containing all active blocks
                let min_x = active_blocks.iter().map(|(x, _)| x).min().unwrap();
                let max_x = active_blocks.iter().map(|(x, _)| x).max().unwrap();
                let min_y = active_blocks.iter().map(|(_, y)| y).min().unwrap();
                let max_y = active_blocks.iter().map(|(_, y)| y).max().unwrap();

                ((min_x + max_x) / 2, (min_y + max_y) / 2)
            }
        };

        let rotation_matrix = [
            [0, 1],
            [-1, 0],
        ];

        let mut new_active_blocks = Vec::new();

        for (x, y) in active_blocks.clone() {
            let x = x as i32 - centre.0 as i32;
            let y = y as i32 - centre.1 as i32;
            let new_x = x * rotation_matrix[0][0] + y * rotation_matrix[0][1];
            let new_y = x * rotation_matrix[1][0] + y * rotation_matrix[1][1];

            let new_x = new_x + centre.0 as i32;
            let new_y = new_y + centre.1 as i32;

            if new_x < 0 || new_x >= ROW_WIDTH as i32 || new_y < 0 || new_y >= ROW_HEIGHT as i32 {
                return;
            }

            new_active_blocks.push((new_x as usize, new_y as usize));
        }

        // Adjust the new active blocks to prevent downward movement
        let max_y = new_active_blocks.iter().map(|&(_, y)| y).max().unwrap();
        if max_y > centre.1 {
            let offset = max_y - centre.1;
            for (_, y) in new_active_blocks.iter_mut() {
            *y += offset;
            }
        }

        println!("{:?}", active_blocks);
        println!("{:?}", new_active_blocks);

        // now check if the new active blocks can be placed
        for (x, y) in new_active_blocks.iter() {
            if self.rows[*y].cells[*x] == Block::Full {
                return;
            }
        }

        // clear the old active blocks
        for (x, y) in active_blocks {
            self.rows[y].cells[x] = Block::Empty;
        }

        // set the new active blocks
        for (x, y) in new_active_blocks {
            self.rows[y].cells[x] = Block::Active;
        }
    }

    fn spawn(&mut self, shape: Shape) -> Result<(), String> {
        match shape {
            Shape::I => {
                let x = ROW_WIDTH / 2;
                let y = 0;
                if self.rows[y].cells[x] == Block::Empty
                    && self.rows[y + 1].cells[x] == Block::Empty
                    && self.rows[y + 2].cells[x] == Block::Empty
                    && self.rows[y + 3].cells[x] == Block::Empty
                {
                    self.set((x, y), Block::Active);
                    self.set((x, y + 1), Block::Active);
                    self.set((x, y + 2), Block::Active);
                    self.set((x, y + 3), Block::Active);
                } else {
                    return Err("Cannot spawn shape".to_string())
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
                    self.set((x, y), Block::Active);
                    self.set((x + 1, y), Block::Active);
                    self.set((x, y + 1), Block::Active);
                    self.set((x + 1, y + 1), Block::Active);
                } else {
                    return Err("Cannot spawn shape".to_string())
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
                    self.set((x, y), Block::Active);
                    self.set((x - 1, y), Block::Active);
                    self.set((x + 1, y), Block::Active);
                    self.set((x, y + 1), Block::Active);
                } else {
                    return Err("Cannot spawn shape".to_string())
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
                    self.set((x, y), Block::Active);
                    self.set((x + 1, y), Block::Active);
                    self.set((x, y + 1), Block::Active);
                    self.set((x - 1, y + 1), Block::Active);
                } else {
                    return Err("Cannot spawn shape".to_string())
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
                    self.set((x, y), Block::Active);
                    self.set((x - 1, y), Block::Active);
                    self.set((x, y + 1), Block::Active);
                    self.set((x + 1, y + 1), Block::Active);
                } else {
                    return Err("Cannot spawn shape".to_string())
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
                    self.set((x, y), Block::Active);
                    self.set((x - 1, y), Block::Active);
                    self.set((x + 1, y), Block::Active);
                    self.set((x + 1, y + 1), Block::Active);
                } else {
                    return Err("Cannot spawn shape".to_string())
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
                    self.set((x, y), Block::Active);
                    self.set((x - 1, y), Block::Active);
                    self.set((x + 1, y), Block::Active);
                    self.set((x - 1, y + 1), Block::Active);
                } else {
                    return Err("Cannot spawn shape".to_string())
                }
            }
        }
        
        self.active_shape = Some(shape);

        Ok(())
    }
}


fn handle_events(grid: &mut Grid) -> Result<bool, String> {
    match event::read() {
        Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => {
                return Ok(true);
            }
            KeyCode::Char('i') => {
                grid.spawn(Shape::I)?;
            }
            KeyCode::Char('o') => {
                grid.spawn(Shape::O)?;
            }
            KeyCode::Char('t') => {
                grid.spawn(Shape::T)?;
            }
            KeyCode::Char('s') => {
                grid.spawn(Shape::S)?;
            }
            KeyCode::Char('z') => {
                grid.spawn(Shape::Z)?;
            }
            KeyCode::Char('j') => {
                grid.spawn(Shape::J)?;
            }
            KeyCode::Char('l') => {
                grid.spawn(Shape::L)?;
            }
            KeyCode::Char('r') => {
                grid.rotate_active_block();
            }
            KeyCode::Char('d') => {
                grid.bring_down();
            }
            // handle other key events
            _ => {}
        },

        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }
    Ok(false)
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<(), String> {
    let mut grid = Grid::new();
    loop {
        terminal
            .draw(|frame| draw(frame, grid))
            .map_err(|e| e.to_string())?;
        if handle_events(&mut grid)? {
            // relinquish keyboard control
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame, grid: Grid) {
    let mut rows = Vec::new();
    for row in grid.rows.iter() {
        let mut line = String::new();
        for cell in row.cells.iter() {
            line.push(match cell {
                Block::Full => '█',
                Block::Empty => ' ',
                Block::Active => 'X',
            });
        }
        rows.push(line);
    }
    let text = rows.join("\n");
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, frame.size());
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    run(&mut terminal);

    // Restore the terminal to its original state
    terminal
        .backend_mut()
        .execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    io::stdout().execute(terminal::LeaveAlternateScreen)?;
    io::stdout().execute(crossterm::event::DisableMouseCapture)?;
    io::stdout().flush()?;

    Ok(())
}

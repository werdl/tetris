use crate::block::{Block, Row};
use crate::shape::Shape;
use crate::constants::{ROW_HEIGHT, ROW_WIDTH};

#[derive(Clone)]
pub struct Grid {
    pub rows: [Row; ROW_HEIGHT],
    pub active_shape: Option<Shape>,
    pub next_shape: Shape,
    pub held_shape: Option<Shape>,
    pub shapes: u32,
    pub score: u32,
    pub level: u32,
    pub paused: bool,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rows: [Row {
                cells: [Block::Empty; ROW_WIDTH],
            }; ROW_HEIGHT],
            active_shape: None,
            next_shape: Shape::random(),
            held_shape: None,
            shapes: 0,
            score: 0,
            level: 1,
            paused: false,
        }
    }

    pub fn next(&mut self, next: Option<Shape>) -> bool {
        self.shapes += 1;
        if self.shapes % 25 == 0 && self.level < 10 {
            self.level += 1;
        }
        self.active_shape = Some(self.next_shape);
        self.next_shape = next.unwrap_or_else(Shape::random);
        self.spawn(self.active_shape.unwrap())
    }

    pub fn set(&mut self, (x, y): (usize, usize), block: Block) {
        self.rows[y].cells[x] = block;
    }

    /// returns whether or not the shape was successfully moved down. any impossibilities are handled by the game end callback provided
    pub fn bring_down(&mut self, next: Option<Shape>, mut end_cb: impl FnMut(Grid) -> ()) -> bool {
        let mut active_blocks = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.cells.iter().enumerate() {
                if let Block::Active(shape) = *cell {
                    active_blocks.push((x, y, shape));
                }
            }
        }

        let mut new_active_blocks = Vec::with_capacity(active_blocks.len());

        for (x, y, shape) in &active_blocks {
            new_active_blocks.push((*x, *y + 1, *shape));
        }

        for (x, y, _) in &new_active_blocks {
            if *y == ROW_HEIGHT - 1 || matches!(self.rows[y + 1].cells[*x], Block::Full(_)) {
                for (x, y, _) in &active_blocks {
                    self.rows[*y].cells[*x] = Block::Empty;
                }
                for (x, y, shape) in &new_active_blocks {
                    self.rows[*y].cells[*x] = Block::Full(*shape);
                }
                
                if self.rows[0].cells.iter().any(|cell| cell.is_full()) || self.rows[1].cells.iter().any(|cell| cell.is_full()) {
                    end_cb(self.clone());
                } else {
                    self.next(next);
                }

                return false;
            }
        }

        for (x, y, _) in &active_blocks {
            self.rows[*y].cells[*x] = Block::Empty;
        }

        for (x, y, shape) in &new_active_blocks {
            self.rows[*y].cells[*x] = Block::Active(*shape);
        }

        if self.rows[0].cells.iter().any(|cell| cell.is_full()) || self.rows[1].cells.iter().any(|cell| cell.is_full()) {
            end_cb(self.clone());
            return false;
        }
        
        true
    }

    pub fn move_active_blocks(&mut self, dx: i32, dy: i32) {
        let mut active_blocks = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.cells.iter().enumerate() {
                if let Block::Active(shape) = *cell {
                    active_blocks.push((x, y, shape));
                }
            }
        }

        let mut new_active_blocks = Vec::with_capacity(active_blocks.len());

        for (x, y, shape) in &active_blocks {
            new_active_blocks.push((
                x.wrapping_add(dx as usize),
                y.wrapping_add(dy as usize),
                *shape,
            ));
        }

        for (x, y, _) in &new_active_blocks {
            if *x >= ROW_WIDTH
                || *y >= ROW_HEIGHT
                || matches!(self.rows[*y].cells[*x], Block::Full(_))
            {
                return;
            }
        }

        for (x, y, _) in &active_blocks {
            self.rows[*y].cells[*x] = Block::Empty;
        }

        for (x, y, shape) in &new_active_blocks {
            self.rows[*y].cells[*x] = Block::Active(*shape);
        }
    }

    pub fn rotate_active_block(&mut self) {
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

        let mut new_active_blocks = Vec::with_capacity(active_blocks.len());

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

        for (x, y, _) in &active_blocks {
            self.rows[*y].cells[*x] = Block::Empty;
        }

        for (x, y, shape) in &new_active_blocks {
            self.rows[*y].cells[*x] = Block::Active(*shape);
        }
    }

    pub fn remove_full_rows(&mut self) {
        let mut full_rows = Vec::new();

        for (y, row) in self.rows.iter().enumerate() {
            if row.cells.iter().all(|cell| matches!(cell, Block::Full(_))) {
                full_rows.push(y);
            }
        }

        let num_full_rows = full_rows.len();
        if num_full_rows > 0 {
            self.score += match num_full_rows {
                1 => 40 * self.level,
                2 => 100 * self.level,
                3 => 300 * self.level,
                4 => 1200 * self.level,
                _ => 0,
            } as u32;
        }

        for row in full_rows.iter() {
            for y in (0..*row).rev() {
                self.rows[y + 1] = self.rows[y].clone();
            }
            self.rows[0] = Row {
                cells: [Block::Empty; ROW_WIDTH],
            };
        }
    }

    pub fn spawn(&mut self, shape: Shape) -> bool {
        self.remove_full_rows();
        let x = ROW_WIDTH / 2;
        let y = 0;

        let positions = match shape {
            Shape::I => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Shape::O => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
            Shape::T => vec![(x, y), (x - 1, y), (x + 1, y), (x, y + 1)],
            Shape::S => vec![(x, y), (x + 1, y), (x, y + 1), (x - 1, y + 1)],
            Shape::Z => vec![(x, y), (x - 1, y), (x, y + 1), (x + 1, y + 1)],
            Shape::J => vec![(x, y), (x - 1, y), (x + 1, y), (x + 1, y + 1)],
            Shape::L => vec![(x, y), (x - 1, y), (x + 1, y), (x - 1, y + 1)],
        };

        for &(x, y) in &positions {
            if self.rows[y].cells[x] != Block::Empty {
                return false;
            }
        }

        for &(x, y) in &positions {
            self.set((x, y), Block::Active(shape));
        }

        self.active_shape = Some(shape);

        // we can safely give a useless callback here
        self.bring_down(None, |_| {});

        true
    }

    pub fn hold(&mut self) {
        if let Some(active_shape) = self.active_shape.take() {
            // replace the active shape with the held shape

            for row in &mut self.rows {
                for cell in &mut row.cells {
                    if let Block::Active(_shape) = cell {
                        *cell = Block::Empty;
                    }
                }
            }

            if let Some(held_shape) = self.held_shape.replace(active_shape) {
                self.spawn(held_shape);
            } else {
                self.next(None);
            }
        }
    }
}

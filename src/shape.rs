use ratatui::{
    style::{Color, Stylize},
    text::Line,
};

// src/shape.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Shape {
    pub fn random() -> Shape {
        use rand::Rng;
        match rand::thread_rng().gen_range(0..7) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            _ => Shape::L,
        }
    }

    pub fn color(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::with_capacity(4);
        let shape_repr = match self {
            Shape::I => crate::constants::I,
            Shape::O => crate::constants::O,
            Shape::T => crate::constants::T,
            Shape::S => crate::constants::S,
            Shape::Z => crate::constants::Z,
            Shape::J => crate::constants::J,
            Shape::L => crate::constants::L,
        };
        for line in shape_repr.split('\n') {
            lines.push(Line::from(line).fg(match self {
                Shape::I => Color::Red,
                Shape::O => Color::Blue,
                Shape::T => Color::Rgb(255, 165, 0),
                Shape::S => Color::Green,
                Shape::Z => Color::Cyan,
                Shape::J => Color::White,
                Shape::L => Color::Magenta,
            }));
        }
        lines
    }
}

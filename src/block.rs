// src/block.rs

use crate::shape::Shape;
use crate::constants::ROW_WIDTH;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Full(Shape),
    Empty,
    Active(Shape),
}

impl Block {
    pub fn repr(&self) -> String {
        match self {
            Block::Full(_) | Block::Active(_) => "██",
            Block::Empty => ". ",
        }.to_string()
    }    

    pub fn is_full(&self) -> bool {
        match self {
            Block::Full(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Row {
    pub cells: [Block; ROW_WIDTH],
}


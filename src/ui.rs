use ratatui::{
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::Paragraph,
    Frame,
};

use crate::{block::Block, constants::{ROW_HEIGHT, ROW_WIDTH}, grid::Grid, shape::Shape};

pub fn draw(frame: &mut Frame, grid: Grid) {
    let mut lines = Vec::with_capacity(ROW_HEIGHT);
    for row in &grid.rows {
        let mut line = Line::raw("");
        for cell in &row.cells {
            let styled_cell = match cell {
                Block::Active(shape) | Block::Full(shape) => match shape {
                    Shape::I => cell.repr().fg(Color::Red),
                    Shape::O => cell.repr().fg(Color::Blue),
                    Shape::T => cell.repr().fg(Color::Rgb(255, 165, 0)),
                    Shape::S => cell.repr().fg(Color::Green),
                    Shape::Z => cell.repr().fg(Color::Cyan),
                    Shape::J => cell.repr().fg(Color::White),
                    Shape::L => cell.repr().fg(Color::Magenta),
                },
                _ => cell.repr().fg(Color::White),
            };
            line.push_span(styled_cell);
        }
        lines.push(Line::from(line));
    }

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
    let help_text = "Controls:
    ←: Move left
    →: Move right
    ↑: Rotate
    ↓: Move down
    q: Quit

Options:
    -l, --level: Start at a specific level (1-10)
    ";

    // Preview of the next shape
    let repr = grid.next_shape.color();

    let mut next_shape_text = vec![
        Line::from("Next Shape:").fg(Color::White),
        Line::from(""),
    ];

    next_shape_text.extend(repr);

    let next_shape_paragraph = Paragraph::new(next_shape_text);
    let next_shape_area = ratatui::layout::Rect::new(
        area.x + 1,
        area.y + area.height - 10,
        20,
        5,
    );
    frame.render_widget(next_shape_paragraph, next_shape_area);

    // Display held shape
    let held_shape_repr = match grid.held_shape {
        Some(shape) => {
            let mut lines = Vec::with_capacity(4);
            let shape_repr = match shape {
                Shape::I => crate::constants::I,
                Shape::O => crate::constants::O,
                Shape::T => crate::constants::T,
                Shape::S => crate::constants::S,
                Shape::Z => crate::constants::Z,
                Shape::J => crate::constants::J,
                Shape::L => crate::constants::L,
            };
            for line in shape_repr.split('\n') {
                lines.push(Line::from(line).fg(match shape {
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
        None => vec![Line::from("None").fg(Color::White)],
    };

    let mut held_shape_text = vec![
        Line::from("Held Shape:").fg(Color::White),
        Line::from(""),
    ];

    held_shape_text.extend(held_shape_repr);

    let held_shape_paragraph = Paragraph::new(held_shape_text);
    let held_shape_area = ratatui::layout::Rect::new(
        area.x + 1,
        area.y + area.height - 20,
        20,
        5,
    );
    frame.render_widget(held_shape_paragraph, held_shape_area);

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



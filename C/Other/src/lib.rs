use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum Vertical {
    UP,
    DOWN,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum Horizontal {
    LEFT,
    RIGHT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Corner {
    vertical: Vertical,
    horizontal: Horizontal,
}

impl Display for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self.vertical, self.horizontal) {
                (Vertical::UP, Horizontal::LEFT) => "┘",
                (Vertical::UP, Horizontal::RIGHT) => "└",
                (Vertical::DOWN, Horizontal::LEFT) => "┐",
                (Vertical::DOWN, Horizontal::RIGHT) => "┌",
            }
        )?;
        Ok(())
    }
}

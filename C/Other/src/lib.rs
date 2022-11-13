use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
enum Vertical {
    Up,
    Down,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
enum Horizontal {
    Left,
    Right,
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
                (Vertical::Up, Horizontal::Left) => "┘",
                (Vertical::Up, Horizontal::Right) => "└",
                (Vertical::Down, Horizontal::Left) => "┐",
                (Vertical::Down, Horizontal::Right) => "┌",
            }
        )?;
        Ok(())
    }
}

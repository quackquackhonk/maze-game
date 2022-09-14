#![allow(non_snake_case)]

use std::{fmt::Display, io};

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
struct Corner {
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

fn main() {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin().lock());
    let iterator = deserializer
        .into_iter::<Corner>()
        .flatten()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&iterator);
    println!("{}", json.unwrap());
}

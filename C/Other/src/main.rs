#![allow(non_snake_case)]

use std::io;

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

impl ToString for Corner {
    fn to_string(&self) -> String {
        String::from(match (self.vertical, self.horizontal) {
            (Vertical::UP, Horizontal::LEFT) => "┘",
            (Vertical::UP, Horizontal::RIGHT) => "└",
            (Vertical::DOWN, Horizontal::LEFT) => "┐",
            (Vertical::DOWN, Horizontal::RIGHT) => "┌",
        })
    }
}

fn main() {
    //let mut buf = String::new();
    //let file_size = io::stdin().read_to_string(&mut buf).unwrap();

    //println!("{buf}")
    //let test = r#"{"vertical" : "UP", "horizontal" : "LEFT" }{"vertical" : "DOWN", "horizontal" : "RIGHT" }"#;

    //test.split_inclusive("}");
    let deserializer = serde_json::Deserializer::from_reader(io::stdin().lock());
    let iterator = deserializer.into_iter::<Corner>();

    println!("[");
    for value in iterator {
        println!("{},", value.unwrap().to_string());
    }
    println!("]")
}

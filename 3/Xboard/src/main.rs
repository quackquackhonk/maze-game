use std::io;

use common::gem::Gem;
use serde::Serialize;

#[derive(Serialize)]
struct Board {
    connectors: Matrix<Connector>,
    treasures: Matrix<Treasure>,
}

#[derive(Serialize)]
struct Matrix<T>([Row<T>; 7]);

#[derive(Serialize)]
struct Row<T>([T; 7]);

#[derive(Serialize)]
enum Connector {
    #[serde(rename(serialize = "│"))]
    VerticalPath,
    #[serde(rename(serialize = "─"))]
    HorizontalPath,
    #[serde(rename(serialize = "┐"))]
    SouthCorner,
    #[serde(rename(serialize = "└"))]
    NorthCorner,
    #[serde(rename(serialize = "┌"))]
    EastCorner,
    #[serde(rename(serialize = "┘"))]
    WestCorner,
    #[serde(rename(serialize = "┬"))]
    SouthFork,
    #[serde(rename(serialize = "┴"))]
    NorthFork,
    #[serde(rename(serialize = "┤"))]
    WestFork,
    #[serde(rename(serialize = "├"))]
    EastFork,
    #[serde(rename(serialize = "┼"))]
    Crossroads,
}

#[derive(Serialize)]
struct Treasure([Gem; 2]);

#[derive(Serialize)]
struct Coordinate {
    row: Index,
    column: Index,
}

#[derive(Serialize)]
struct Index(usize);

fn main() {
    println!("Hello, world!");
}

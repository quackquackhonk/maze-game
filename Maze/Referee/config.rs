use std::collections::VecDeque;

use common::grid::Position;

#[derive(Debug)]
pub struct Config {
    /// If None use single goal model.
    /// If Some() use multiple goals
    multiple_goals: Option<VecDeque<Position>>,
}

impl Config {
    pub fn multiple_goals_mut(&mut self) -> &mut Option<VecDeque<Position>> {
        &mut self.multiple_goals
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            multiple_goals: None,
        }
    }
}

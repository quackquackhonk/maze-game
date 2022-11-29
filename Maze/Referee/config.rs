#[derive(Debug)]
pub struct Config {
    /// If None use single goal model.
    /// If Some() use multiple goals
    pub multiple_goals: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            multiple_goals: false,
        }
    }
}

/// contains data defintions for remote messages
pub mod json;
pub mod player;
pub mod referee;

pub fn is_port(s: &str) -> Result<usize, String> {
    let port_number: usize = s.parse().map_err(|e| format!("{}", e))?;
    if !(10000..=60000).contains(&port_number) {
        return Err("Port Number must be between 10000 and 60000 inclusive".to_string());
    }
    Ok(port_number)
}

use std::{io::Write, net::TcpStream};

use clap::Parser;
use common::json::Name;
use players::{player::LocalPlayer, strategy::NaiveStrategy};
use remote::is_port;
use remote::referee::RefereeProxy;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "Bill")]
    /// The name of the player you use to connect
    name: Name,

    #[clap(value_parser = is_port)]
    /// The port number the client should connect to
    port: usize,

    #[clap(short, long, value_parser = parse_strategy, default_value = "Euclid")]
    strategy: NaiveStrategy,
}

fn parse_strategy(s: &str) -> Result<NaiveStrategy, String> {
    match s.to_lowercase().as_str() {
        "euclid" => Ok(NaiveStrategy::Euclid),
        "riemann" => Ok(NaiveStrategy::Riemann),
        _ => Err("Not a valid strategy".to_string()),
    }
}

fn main() -> anyhow::Result<()> {
    let Args {
        name,
        port,
        strategy,
    } = Args::parse();
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;
    stream.write_all(serde_json::to_string(&name)?.as_bytes())?;
    let player = Box::new(LocalPlayer::new(name, strategy));
    let referee = RefereeProxy::from_tcp(player, stream);
    referee.listen()?;
    Ok(())
}

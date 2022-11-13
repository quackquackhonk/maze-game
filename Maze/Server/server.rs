use common::json::Name;
use referee::json::JsonGameResult;
use referee::referee::GameResult;
use remote::player::PlayerProxy;
use serde::Deserialize;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use tokio::time::timeout;

async fn recieve_connections(
    listener: &TcpListener,
    connections: &mut Vec<PlayerProxy<TcpStream, TcpStream>>,
) -> io::Result<()> {
    loop {
        let (stream, _) = listener.accept()?;
        let player = PlayerProxy::try_from_tcp(stream)?;
        connections.push(player);
        if connections.len() == 6 {
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut player_connections: Vec<PlayerProxy<TcpStream, TcpStream>> = vec![];

    let time_out = timeout(
        Duration::from_secs(20),
        recieve_connections(&listener, &mut player_connections),
    );
    // we have enough players :)

    Ok(())
}

use clap::Parser;
use common::grid::Position;
use common::json::Name;
use common::{FullPlayerInfo, State};
use players::player::PlayerApi;
use referee::json::JsonRefereeState;
use referee::player::Player;
use referee::referee::{GameResult, Referee};
use remote::player::PlayerProxy;
use serde::Deserialize;
use std::io::stdin;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;

const TIMEOUT: Duration = Duration::from_secs(20);

const NUM_WAITING_PERIODS: u64 = 2;

#[derive(Parser)]
struct Args {
    port: u16,
}

/// Given a tokio TcpStream, attempts to create a `PlayerProxy` from that stream.
fn create_player(
    stream: tokio::net::TcpStream,
) -> anyhow::Result<PlayerProxy<TcpStream, TcpStream>> {
    let stream = stream.into_std()?;

    stream.set_nonblocking(false)?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("We did not pass a 0 for duration");

    let name_stream = stream.try_clone()?;
    let name = Name::deserialize(&mut serde_json::Deserializer::from_reader(name_stream))?;

    Ok(PlayerProxy::try_from_tcp(name, stream)?)
}

async fn recieve_connections(
    listener: &TcpListener,
    connections: &mut Vec<Box<dyn PlayerApi>>,
    num_players: usize,
) {
    while connections.len() < num_players {
        if let Ok((stream, _)) = listener.accept().await {
            if let Ok(player) = create_player(stream) {
                connections.push(Box::new(player));
                eprintln!("Player #{} connected", connections.len());
            }
        };
    }
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let Args { port } = Args::parse();

    eprintln!("Parsing JsonRefereeState");
    let (state_info, goals): (State<FullPlayerInfo>, Vec<Position>) = {
        let jsonstate: JsonRefereeState = serde_json::from_reader(stdin())?;
        jsonstate.try_into()?
    };
    let num_players = state_info.player_info.len();

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).await?;
    eprintln!("Bound to port: {port}");
    let mut player_connections: Vec<Box<dyn PlayerApi>> = vec![];

    for _ in 0..NUM_WAITING_PERIODS {
        let time_out = timeout(
            TIMEOUT,
            recieve_connections(&listener, &mut player_connections, num_players),
        );
        if (time_out.await).is_ok() {
            break;
        }
    }

    if player_connections.len() < num_players || player_connections.len() < 2 {
        // We waited twice and there is not enough players
        let game_result = GameResult::default();
        println!("{}", serde_json::to_string(&game_result).unwrap());
        return Ok(());
    }

    let mut state = State {
        board: state_info.board,
        player_info: player_connections
            .into_iter()
            .rev()
            .zip(state_info.player_info)
            .map(|(api, info)| Player::new(api, info))
            .collect(),
        previous_slide: state_info.previous_slide,
    };

    // we have enough players :)
    let mut referee = Referee::new(1);
    let mut game_result = referee.run_from_state(&mut state, &mut vec![], goals.into());
    game_result.winners.sort();
    game_result.kicked.sort();
    println!("{}", serde_json::to_string(&game_result).unwrap());

    Ok(())
}

use clap::Parser;
use common::{FullPlayerInfo, State};
use players::player::PlayerApi;
use referee::json::JsonRefereeState;
use referee::player::Player;
use referee::referee::{GameResult, Referee};
use remote::is_port;
use remote::player::PlayerProxy;
use std::io::{self, stdin};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;

const TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Parser)]
struct Args {
    #[clap(value_parser = is_port)]
    port: u16,
}

async fn recieve_connections(
    listener: &TcpListener,
    connections: &mut Vec<Box<dyn PlayerApi>>,
    num_players: usize,
) -> io::Result<()> {
    while connections.len() < num_players {
        if let Ok((stream, _)) = listener.accept().await {
            let stream = stream
                .into_std()
                .expect("could not convert async stream to sync");
            stream
                .set_nonblocking(false)
                .expect("set set_nonblocking call failed");
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .expect("We did not pass a 0 for duration");

            if let Ok(player) = PlayerProxy::try_from_tcp(stream) {
                connections.push(Box::new(player));
                eprintln!("Player #{} connected", connections.len());
            }
        };
    }
    Ok(())
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let Args { port } = Args::parse();

    eprintln!("Parsing JsonRefereeState");
    let state_info: State<FullPlayerInfo> = {
        let jsonstate: JsonRefereeState = serde_json::from_reader(stdin())?;
        jsonstate.into()
    };
    let num_players = state_info.player_info.len();

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).await?;
    eprintln!("Bound to port: {port}");
    let mut player_connections: Vec<Box<dyn PlayerApi>> = vec![];

    let time_out = timeout(
        TIMEOUT,
        recieve_connections(&listener, &mut player_connections, num_players),
    );

    if time_out.await.is_err() {
        eprintln!("Timed out with only {} players", player_connections.len());
        // We timed out once but did not have enough players

        let time_out = timeout(
            TIMEOUT,
            recieve_connections(&listener, &mut player_connections, num_players),
        );

        if time_out.await.is_err() {
            eprintln!(
                "Timed out again with only {} players, ending the game",
                player_connections.len()
            );
            // We waited twice and there is not enough players
            let game_result = GameResult::default();
            println!("{}", serde_json::to_string(&game_result).unwrap());
            return Ok(());
        }
    }

    let mut state = State {
        board: state_info.board,
        player_info: state_info
            .player_info
            .into_iter()
            .zip(player_connections)
            .map(|(info, api)| Player::new(api, info))
            .collect(),
        previous_slide: state_info.previous_slide,
    };

    // we have enough players :)
    let mut referee = Referee::new(1);
    let game_result = referee.run_from_state(&mut state, &mut vec![]);
    println!("{}", serde_json::to_string(&game_result).unwrap());

    Ok(())
}

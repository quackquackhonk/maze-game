use clap::Parser;
use common::{FullPlayerInfo, State};
use players::player::PlayerApi;
use referee::json::JsonRefereeState;
use referee::player::Player;
use referee::referee::{GameResult, Referee};
use remote::is_port;
use remote::player::PlayerProxy;
use std::io::{self, stdin};
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;
use tokio::task::yield_now;
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
) -> io::Result<()> {
    listener.set_nonblocking(true)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .expect("We did not pass a 0 for duration");
                if let Ok(player) = PlayerProxy::try_from_tcp(stream) {
                    connections.push(Box::new(player));
                    eprintln!("Player #{} connected", connections.len());
                    if connections.len() == 6 {
                        break;
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                yield_now().await //We await here so that we give up execution
                                  //to tokio for it to decide if we have reached our timeout
            }
            Err(e) => Err(e)?,
        }
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

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port)))?;
    eprintln!("Bound to port: {port}");
    let mut player_connections: Vec<Box<dyn PlayerApi>> = vec![];

    let time_out = timeout(
        TIMEOUT,
        recieve_connections(&listener, &mut player_connections),
    );

    if time_out.await.is_err() && player_connections.len() < 2 {
        eprintln!("timed out with only {} players", player_connections.len());
        // We timed out once but did not have enough players

        let time_out = timeout(
            TIMEOUT,
            recieve_connections(&listener, &mut player_connections),
        );

        if time_out.await.is_err() && player_connections.len() < 2 {
            eprintln!(
                "timed out again with only {} players, ending the game",
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

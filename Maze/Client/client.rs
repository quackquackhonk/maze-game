use std::io::stdin;
use std::net::IpAddr;
use std::thread;
use std::time::Duration;
use std::{io::Write, net::TcpStream};

use clap::Parser;
use players::bad_player::{BadPlayer, BadPlayerLoop};
use players::player::LocalPlayer;
use players::player::PlayerApi;
use referee::json::PlayerSpec;
use remote::referee::RefereeProxy;

#[derive(Parser)]
struct Args {
    /// The port number the client should connect to
    port: u16,

    #[clap(default_value = "127.0.0.1")]
    address: IpAddr,
}

fn main() -> anyhow::Result<()> {
    let Args { port, address } = Args::parse();

    let players: Vec<PlayerSpec> = serde_json::from_reader(stdin())?;
    thread::scope(|s| {
        for ps in players.into_iter().rev() {
            s.spawn(|| {
                let (player, name): (Box<dyn PlayerApi>, _) = match ps {
                    PlayerSpec::PS(ps) => {
                        let (name, strategy) = ps.into();
                        (Box::new(LocalPlayer::new(name.clone(), strategy)), name)
                    }
                    PlayerSpec::BadPS(badps) => {
                        let (name, strategy, bad_fm) = badps.into();
                        (
                            Box::new(BadPlayer::new(
                                Box::new(LocalPlayer::new(name.clone(), strategy)),
                                bad_fm,
                            )),
                            name,
                        )
                    }
                    PlayerSpec::BadPS2(badps2) => {
                        let (name, strategy, badfm, times) = badps2.into();
                        (
                            Box::new(BadPlayerLoop::new(
                                Box::new(LocalPlayer::new(name.clone(), strategy)),
                                badfm,
                                times,
                            )),
                            name,
                        )
                    }
                };
                eprintln!("Started client");
                let mut stream = {
                    loop {
                        if let Ok(stream) = TcpStream::connect((address, port)) {
                            eprintln!("Connected to server");
                            break stream;
                        }
                    }
                };
                stream.write_all(serde_json::to_string(&name)?.as_bytes())?;
                let mut referee = RefereeProxy::from_tcp(player, stream);
                referee.listen()
            });
            thread::sleep(Duration::from_secs(3));
        }
    });

    Ok(())
}

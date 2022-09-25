#![allow(non_snake_case)]
use std::{io::Read, net::TcpListener};

use clap::Parser;
use xjson::Corner;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser = is_port)]
    /// The port number the server should listen on
    port: usize,
}

fn is_port(s: &str) -> Result<usize, String> {
    let port_number: usize = s.parse().map_err(|e| format!("{}", e))?;
    if port_number < 10000 || port_number > 60000 {
        return Err("Port Number must be between 10000 and 60000 inclusive".to_string());
    }
    Ok(port_number)
}

fn main() -> std::io::Result<()> {
    let Args { port } = Args::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    if let Some(stream) = listener.incoming().nth(0) {
        let response = handle_client(stream?);
        println!("{response}");
    }

    Ok(())
}

fn handle_client(stream: impl Read) -> String {
    let deserializer = serde_json::Deserializer::from_reader(stream);
    let corners = deserializer
        .into_iter::<Corner>()
        .flatten()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();
    let json = serde_json::to_string(&corners);
    json.unwrap()
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader, path::Path};

    use crate::handle_client;

    #[test]
    fn test_handle_client_no_data() {
        let reader = "".as_bytes();
        let response = handle_client(reader);
        assert_eq!(response, String::from("[]"));
    }

    #[test]
    fn test_handle_client_from_file() {
        enum Filetype {
            INPUT,
            OUTPUT,
            INVALID,
        }
        use Filetype::*;
        let tests_path = Path::new("./../Tests/");
        let files = tests_path.read_dir().unwrap().collect::<Vec<_>>();
        let file_count = files.iter().count() / 2;
        let mut results: Vec<(Option<String>, Option<String>)> = vec![(None, None); file_count];
        for test in files {
            if let Ok(dir_entry) = test {
                let path = dir_entry.path();
                let mut split_path = path.file_name().unwrap().to_str().unwrap().split("-");
                if let Ok(num) = split_path.next().unwrap().parse::<usize>() {
                    let is_input_file = match split_path.next() {
                        Some("in.json") => INPUT,
                        Some("out.json") => OUTPUT,
                        _ => INVALID,
                    };
                    match is_input_file {
                        INPUT => {
                            results[num].0 =
                                Some(handle_client(BufReader::new(File::open(&path).unwrap())));
                        }
                        OUTPUT => results[num].1 = Some(std::fs::read_to_string(&path).unwrap()),
                        INVALID => {}
                    };
                }
            }
        }

        for (input, output) in results {
            let input = input
                .iter()
                .map(|str| serde_json::from_str(&str).unwrap())
                .collect::<Vec<serde_json::Value>>();
            let output = output
                .iter()
                .map(|str| serde_json::from_str(&str).unwrap())
                .collect::<Vec<serde_json::Value>>();
            assert_eq!(input, output);
        }
    }
}

#![allow(non_snake_case)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

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
    if !(10000..=60000).contains(&port_number) {
        return Err("Port Number must be between 10000 and 60000 inclusive".to_string());
    }
    Ok(port_number)
}

fn main() -> std::io::Result<()> {
    let Args { port } = Args::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    if let Some(stream) = listener.incoming().nth(0) {
        let mut stream = stream?;
        let response = read_from_client(&mut stream)?;
        write_to_client(&mut stream, &format!("{response}\n"))?;
    }

    Ok(())
}

fn write_to_client(stream: &mut impl Write, response: &str) -> std::io::Result<usize> {
    stream.write(response.as_bytes())
}

fn read_from_client(stream: &mut impl Read) -> Result<String, serde_json::Error> {
    let deserializer = serde_json::Deserializer::from_reader(stream);
    let corners = deserializer
        .into_iter::<Corner>()
        .flatten()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();
    serde_json::to_string(&corners)
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader, path::Path};

    use crate::read_from_client;

    #[test]
    fn test_handle_client_no_data() {
        let mut reader = "".as_bytes();
        let response = read_from_client(&mut reader);
        assert_eq!(response.unwrap(), String::from("[]"));
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
                            results[num].0 = Some(
                                read_from_client(&mut BufReader::new(File::open(&path).unwrap()))
                                    .unwrap(),
                            );
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

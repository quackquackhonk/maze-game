use std::io::{stdin, stdout};
use xbad2::*;

fn main() -> anyhow::Result<()> {
    read_and_write_json(stdin().lock(), &mut stdout().lock())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_handle_client_from_file() {
        let tests_path = Path::new("./../Tests/");
        let files = tests_path.read_dir().unwrap().collect::<Vec<_>>();
        let file_count = files.len() / 2;
        let mut results: Vec<(Option<String>, Option<String>)> = vec![(None, None); file_count];
        for dir_entry in files.into_iter().flatten() {
            let path = dir_entry.path();
            let mut split_path = path.file_name().unwrap().to_str().unwrap().split('-');
            if let Ok(num) = split_path.next().unwrap().parse::<usize>() {
                match split_path.next() {
                    Some("in.json") => {
                        let mut buf = Vec::new();
                        read_and_write_json(
                            &mut BufReader::new(File::open(&path).unwrap()),
                            &mut buf,
                        )
                        .map_err(|e| {
                            println!("{}", e);
                            e
                        })
                        .unwrap();

                        results[num].0 = Some(String::from_utf8(buf).unwrap());
                    }
                    Some("out.json") => {
                        results[num].1 = Some(std::fs::read_to_string(&path).unwrap())
                    }
                    _ => {}
                };
            }
        }

        for (input, output) in results {
            let input = input
                .iter()
                .map(|str| serde_json::from_str(str).unwrap())
                .collect::<Vec<serde_json::Value>>();
            let output = output
                .iter()
                .map(|str| serde_json::from_str(str).unwrap())
                .collect::<Vec<serde_json::Value>>();
            assert_eq!(input, output);
        }
    }
}

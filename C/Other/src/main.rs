#![allow(non_snake_case)]

use std::io;

fn main() {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin().lock());
    let iterator = deserializer
        .into_iter::<xjson::Corner>()
        .flatten()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&iterator);
    println!("{}", json.unwrap());
}

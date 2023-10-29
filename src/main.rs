use std::env;

// Available if you need it!
// use serde_bencode

mod bencode;

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            let encoded_value = &args[2];
            let (decoded_value, _) = bencode::decode(encoded_value)?;
            println!("{decoded_value}");
        }
        _ => println!("unknown command: {command}")
    }

    Ok(())
}

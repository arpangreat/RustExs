use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Hi this is myopen , give me a file path");
        }

        2 => {
            if args[1] == "h" {
                println!("help");
            } else {
                let file = File::open(&args[1])?;
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();

                let _ = buf_reader.read_to_string(&mut contents);

                println!("{}", contents);
            }
        }

        _ => {
            todo!();
        }
    }

    Ok(())
}

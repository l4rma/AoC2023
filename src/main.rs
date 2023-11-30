mod day0;
mod utils;

use std::env::args;

fn main() {
    let day = args().nth(1).expect("Usage: cargo run -- <day>");

    match day.trim() {
        "day0" => day0::main(),
        _ => {
            println!("Undefined day");
        }
    }
}

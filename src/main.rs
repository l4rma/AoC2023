mod day0;
mod day1;
mod day2;
mod day3;
mod utils;

use std::env::args;

fn main() {
    let day = args().nth(1).expect("Usage: cargo run -- <day>");

    match day.trim() {
        "day0" => day0::main(),
        "day1" => day1::main(),
        "day2" => day2::main(),
        "day3" => day3::main(),
        _ => {
            println!("Undefined day");
        }
    }
}

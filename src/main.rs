use crate::day1::solve;
use crate::input::pull_input;

use dotenv::dotenv;
use std::env;

mod day1;
mod input;

fn main() {
    dotenv().ok();
    let session_string: String = env::var("session").unwrap();
    let session_token: &str = session_string.as_str();
    let inputs: String = pull_input(2023, 1, session_token);
    println!("received body: {:?}", inputs);
    let lines: Vec<&str> = inputs.split('\n').collect();
    println!("lines: {:?}", lines);
    let result: u32 = solve(lines.as_slice());
    println!("Solution for day {} is {}", 1, result);
}
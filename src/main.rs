use crate::input::pull_input;

use dotenv::dotenv;
use std::env;
use crate::day2::{solve, solve_power};

mod day1;
mod day2;
mod input;

fn main() {
    dotenv().ok();
    let session_string: String = env::var("session").unwrap();
    let session_token: &str = session_string.as_str();
    let inputs: String = pull_input(2023, 2, session_token);
    println!("received body: {:?}", inputs);
    let lines: Vec<&str> = inputs.split('\n').collect();
    println!("lines: {:?}", lines);
    let result: u32 = solve(lines.as_slice());
    println!("Solution for day {} is {}", 2, result);
    let result2: u32 = solve_power(lines.as_slice());
    println!("Solution for day {} is {}", 2.2, result2);
}
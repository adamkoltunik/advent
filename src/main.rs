use crate::input::pull_input;

use dotenv::dotenv;
use std::env;

mod input;

fn main() {
    dotenv().ok();
    let session_string: String = env::var("session").unwrap();
    let session_token: &str = session_string.as_str();
    let inputs: String = pull_input(2023, 1, session_token);
    println!("received body: {:?}", inputs);
    let lines: Vec<&str> = inputs.split('\n').collect();
    println!("lines: {:?}", lines);
}
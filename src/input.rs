use curl;
use curl::easy::{Easy, Transfer};

pub fn pull_input(year: u16, day: u8, session: &str) -> String {
    let mut easy: Easy = Easy::new();
    let url: String = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    easy.cookie(format!("session={}", session).as_str()).unwrap();
    let mut data: Vec<u8> = Vec::new();
    easy.url(url.as_str()).unwrap();
    {
        let mut transfer: Transfer = easy.transfer();
        transfer.write_function(|part: &[u8]| {
            data.extend_from_slice(part);
            Ok(part.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    return String::from_utf8(data).expect("invalid utf-8");
}
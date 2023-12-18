use regex::{Regex};

pub fn solve(lines: &[&str]) -> u32 {
    return lines.iter()
        .map(|line| solve_line(line))
        .sum();
}

pub fn solve_line(line: &str) -> u32 {
    if line == "" {
        return 0;
    }
    println!("input: {:?}", line);
    let regex_game = Regex::new("Game ([0-9]*)").unwrap();
    let game_id: u32 = regex_game.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
    let regex_red = Regex::new("([0-9]*) (red|green|blue)").unwrap();
    for (_, [amount, color]) in regex_red.captures_iter(line).map(|capture| capture.extract()) {
        let amount = amount.parse::<u32>().unwrap();
        match color {
            "red" => if amount > 12 { return 0 }
            "green" => if amount > 13 { return 0 }
            "blue" => if amount > 14 { return 0 }
            _ => {}
        }
    }
    return game_id;
}

pub fn solve_power(lines: &[&str]) -> u32 {
    return lines.iter()
        .map(|line| solve_power_line(line))
        .sum();
}

pub fn solve_power_line(line: &str) -> u32 {
    if line == "" {
        return 0;
    }
    println!("input: {:?}", line);
    let regex_red = Regex::new("([0-9]*) (red|green|blue)").unwrap();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for (_, [amount, color]) in regex_red.captures_iter(line).map(|capture| capture.extract()) {
        let amount = amount.parse::<u32>().unwrap();
        match color {
            "red" => if amount > max_red { max_red = amount }
            "green" => if amount > max_green { max_green = amount }
            "blue" => if amount > max_blue { max_blue = amount }
            _ => {}
        }
    }
    return max_red * max_blue * max_green;
}

#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        test_num(0);
    }

    #[test]
    fn test_2() {
        test_num(1);
    }

    #[test]
    fn test_3() {
        test_num(2);
    }

    #[test]
    fn test_4() {
        test_num(3);
    }

    #[test]
    fn test_5() {
        test_num(4);
    }

    fn test_num(pos: usize){
        let data = test_data()[pos];
        let result = solve_line(data);
        assert_eq!(result, lines_result()[pos]);
    }

    #[test]
    fn test_all() {
        let result = solve(test_data());
        assert_eq!(result, all_result());
    }

    pub fn test_data<'a>() -> &'a[&'a str] {
        return &["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ];
    }

    pub fn lines_result() -> [u32; 5] {
        return [1, 2, 0, 0, 5];
    }

    pub fn all_result() -> u32 {
        8u32
    }
}

#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_1() {
        test_num(0);
    }

    #[test]
    fn test_2() {
        test_num(1);
    }

    #[test]
    fn test_3() {
        test_num(2);
    }

    #[test]
    fn test_4() {
        test_num(3);
    }

    #[test]
    fn test_5() {
        test_num(4);
    }

    fn test_num(pos: usize){
        let data = test_data()[pos];
        let result = solve_power_line(data);
        assert_eq!(result, lines_result()[pos]);
    }

    #[test]
    fn test_all() {
        let result = solve_power(test_data());
        assert_eq!(result, all_result());
    }

    pub fn test_data<'a>() -> &'a[&'a str] {
        return &["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ];
    }

    pub fn lines_result() -> [u32; 5] {
        return [48, 12, 1560, 630, 36];
    }

    pub fn all_result() -> u32 {
        2286u32
    }
}
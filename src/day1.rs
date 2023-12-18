use regex::{Captures, Regex};

pub fn solve(lines: &[&str]) -> u32 {
    return lines.iter().map(|line| solve_line(line)).sum();
}

pub fn solve_line(line: &str) -> u32 {
    if line == "" {
        return 0;
    }
    println!("original: {:?}", line);
    let converted = substitute_str_num(line);
    println!("converted: {:?}", converted);
    let first_find: Option<usize> = converted.find(|char: char| '9' >= char && char >= '0');
    let first_num_pos: usize = first_find.unwrap();
    let first_num = converted.as_bytes()[first_num_pos] as u32 - '0' as u32;
    let last_find: Option<usize> = converted.rfind(|char: char| '9' >= char && char >= '0');
    let last_num_pos: usize = last_find.unwrap();
    let last_num: u32 = converted.as_bytes()[last_num_pos] as u32 - '0' as u32;
    println!("first: {:?}, second: {:?}", first_num, last_num);
    return first_num * 10 + last_num;
}

pub fn substitute_str_num(line: &str) -> String {
    let regex = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let result = regex.replace_all(line, |capture: &Captures| {
        match &capture[0] {
            "one" => "on1e",
            "two" => "tw2o",
            "three" => "th3ree",
            "four" => "fo4ur",
            "five" => "fi5ve",
            "six" => "si6x",
            "seven" => "sev7en",
            "eight" => "ei8ght",
            "nine" => "ni9ne",
            _ => panic!("We should never get here"),
        }
    }
    ).to_string();
    let result2 = regex.replace_all(result.as_str(), |capture: &Captures| {
        match &capture[0] {
            "one" => "on1e",
            "two" => "tw2o",
            "three" => "th3ree",
            "four" => "fo4ur",
            "five" => "fi5ve",
            "six" => "si6x",
            "seven" => "sev7en",
            "eight" => "ei8ght",
            "nine" => "ni9ne",
            _ => panic!("We should never get here"),
        }
    }
    ).to_string();
    return result2;
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
    fn test_all() {
        let result = solve(test_data());
        assert_eq!(result, all_result());
    }

    fn test_num(pos: usize){
        let data = test_data()[pos];
        let result = solve_line(data);
        assert_eq!(result, lines_result()[pos]);
    }

    pub fn test_data<'a>() -> &'a[&'a str] {
        return &["1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet"];
    }

    pub fn lines_result() -> [u32; 4] {
        return [12u32, 38u32, 15u32, 77u32];
    }

    pub fn all_result() -> u32 {
        142u32
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

    #[test]
    fn test_6() {
        test_num(5);
    }

    #[test]
    fn test_7() {
        test_num(6);
    }

    #[test]
    fn test_all() {
        let result = solve(test_data());
        assert_eq!(result, all_result());
    }

    fn test_num(pos: usize){
        let data = test_data()[pos];
        let result = solve_line(data);
        assert_eq!(result, lines_result()[pos]);
    }

    #[test]
    fn test_sevenine() {
        let result = solve_line("sevenine");
        assert_eq!(result, 79);
    }

    #[test]
    fn substitute_str_as_num() {
        let nums = [
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine"];
        let subs = [
            "on1e",
            "tw2o",
            "th3ree",
            "fo4ur",
            "fi5ve",
            "si6x",
            "sev7en",
            "ei8ght",
            "ni9ne"];
        for (pos, sub) in nums.iter().enumerate() {
            let result = substitute_str_num(format!("asdf{}qwer", sub).as_str());
            assert_eq!(result, format!("asdf{}qwer", subs[pos]));
        }
    }


    pub fn test_data<'a>() -> &'a[&'a str] {
        return &[
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen"
        ];
    }

    pub fn lines_result() -> [u32; 7] {
        return [29, 83, 13, 24, 42, 14, 76];
    }

    pub fn all_result() -> u32 {
        281u32
    }
}
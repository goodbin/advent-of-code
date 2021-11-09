use std::str::FromStr;

use adventofcode::*;
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT_FILE: &str = "inputs/day2.txt";
static PWD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([0-9]+?)-([0-9]+?) ([[:alpha:]]{1}): (.*)").unwrap());

struct Password {
    data: String,
    chr: String,
    num1: usize,
    num2: usize,
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = PWD_REGEX.captures(s).unwrap();
        if let (Some(n1), Some(n2), Some(c), Some(pwd)) =
            (cap.get(1), cap.get(2), cap.get(3), cap.get(4))
        {
            let n1 = n1
                .as_str()
                .parse::<usize>()
                .map_err(|e| Error::Parse(e.to_string()))?;
            let n2 = n2
                .as_str()
                .parse::<usize>()
                .map_err(|e| Error::Parse(e.to_string()))?;
            Ok(Password {
                data: String::from(pwd.as_str()),
                chr: String::from(c.as_str()),
                num1: n1,
                num2: n2,
            })
        } else {
            Err(Error::Parse(String::from("Invalid password")))
        }
    }
}

fn password_validate_one(pwd: &Password) -> bool {
    let count = pwd.data.matches(&pwd.chr).count();
    count >= pwd.num1 && count <= pwd.num2
}

fn password_validate_two(pwd: &Password) -> bool {
    let ch1 = pwd.data.get(pwd.num1 - 1..pwd.num1);
    let ch2 = pwd.data.get(pwd.num2 - 1..pwd.num2);
    if let (Some(ch1), Some(ch2)) = (ch1, ch2) {
        (ch1 == pwd.chr) ^ (ch2 == pwd.chr)
    } else {
        false
    }
}

fn main() {
    let data: Vec<Password> = fs::parse_input(INPUT_FILE, "\n").unwrap();

    let answer1 = data.iter().filter(|p| password_validate_one(p)).count();
    println!("answer1: {:?}", answer1);

    let answer2 = data.iter().filter(|p| password_validate_two(p)).count();
    println!("answer2: {:?}", answer2);
}

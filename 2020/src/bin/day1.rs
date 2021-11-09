use adventofcode::*;

const INPUT_FILE: &str = "inputs/day1.txt";
const YEAR: u32 = 2020;

fn main() {
    let numbers: Vec<u32> = fs::parse_input(INPUT_FILE, "\n").unwrap();
    let mut comb = combinator::pairs(numbers.clone());
    let res = comb.find(|(a, b)| a + b == YEAR).map(|(a, b)| a * b);
    if let Some(ans) = res {
        println!("answer 1: {:?}", ans);
    } else {
        println!("unknown");
    }

    let mut comb = combinator::trits(numbers);
    let res = comb
        .find(|(a, b, c)| a + b + c == YEAR)
        .map(|(a, b, c)| a * b * c);
    if let Some(ans) = res {
        println!("answer 2: {:?}", ans);
    } else {
        println!("unknown");
    }
}

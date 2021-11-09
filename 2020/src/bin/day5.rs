use adventofcode::*;
use std::collections::HashSet;

const INPUT_FILE: &str = "inputs/day5.txt";

fn calculate_num(route: &str, max: usize) -> usize {
    let mut begin = 0;
    let mut end = max;
    let mut ret = 0;
    for c in route.chars() {
        let half = ((end - begin) as f64 / 2.0).ceil() as usize;
        match c {
            'F' | 'L' => {
                end -= half;
                ret = begin;
            }
            'B' | 'R' => {
                begin += half;
                ret = end;
            }
            _ => {}
        }
    }
    ret
}

fn calculate_seat(route: &str) -> (usize, usize) {
    let row = calculate_num(&route[0..7], 127);
    let col = calculate_num(&route[7..], 7);
    (row, col)
}

fn main() {
    let data: Vec<String> = fs::parse_input(INPUT_FILE, "\n").unwrap();
    let seats: HashSet<_> = data.iter().map(|route| calculate_seat(route)).collect();
    let id_max = seats.iter().map(|s| s.0 * 8 + s.1).max().unwrap();
    println!("answer1 = {:?}", id_max);

    let mut skip_begin = true;
    let mut answer2 = 0;
    'main: for row in 0..128 {
        for col in 0..8 {
            if !seats.contains(&(row, col)) {
                if !skip_begin {
                    answer2 = row * 8 + col;
                    break 'main;
                }
            } else {
                skip_begin = false;
            }
        }
    }

    println!("answer2 = {:?}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calculate_id(route: &str) -> usize {
        let seat = calculate_seat(route);
        seat.0 * 8 + seat.1
    }

    #[test]
    fn test_answer_one() {
        assert_eq!(calculate_id("BFFFBBFRRR"), 567);
        assert_eq!(calculate_id("FFFBBBFRRR"), 119);
        assert_eq!(calculate_id("BBFFBBFRLL"), 820);
    }
}

use adventofcode::*;
use std::collections::HashSet;
use std::str::FromStr;

const INPUT_FILE: &str = "inputs/day6.txt";

#[derive(Debug)]
struct Group {
    persons: Vec<HashSet<char>>,
}

impl FromStr for Group {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let persons = s.split('\n').map(|a| a.chars().collect()).collect();
        Ok(Group { persons })
    }
}

impl Group {
    fn count_anyone(&self) -> usize {
        self.persons
            .iter()
            .fold(HashSet::<char>::new(), |mut acc, p| {
                acc.extend(p);
                acc
            })
            .len()
    }

    fn count_everyone(&self) -> usize {
        let seed = self.persons.get(0).cloned().unwrap_or(HashSet::new());
        self.persons
            .iter()
            .skip(1)
            .fold(seed, |mut acc, p| acc.intersection(p).cloned().collect())
            .len()
    }
}

fn main() {
    let data: Vec<Group> = fs::parse_input(INPUT_FILE, "\n\n").unwrap();
    println!(
        "answer1 = {:?}",
        data.iter().fold(0, |acc, g| acc + g.count_anyone())
    );
    println!(
        "answer2 = {:?}",
        data.iter().fold(0, |acc, g| acc + g.count_everyone())
    );
}

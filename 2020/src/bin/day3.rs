use adventofcode::*;

const INPUT_FILE: &str = "inputs/day3.txt";

#[derive(Debug)]
enum Cell {
    Tree,
    Space,
}

struct Map {
    data: Vec<Cell>,
    width: usize,
}

impl Map {
    fn get(&self, h: usize, w: usize) -> Option<&Cell> {
        let width = w % self.width;
        let pos = h * self.width + width;
        self.data.get(pos)
    }
}

struct Router<'a> {
    pos: (usize, usize),
    map: &'a Map,
}

impl<'a> Router<'a> {
    fn left(&mut self, v: usize) {
        self.pos.0 += v;
    }

    fn down(&mut self, v: usize) {
        self.pos.1 += v;
    }

    fn current(&self) -> Option<&Cell> {
        self.map.get(self.pos.0, self.pos.1)
    }

    fn new(map: &'a Map) -> Self {
        Self { pos: (0, 0), map }
    }
}

fn count_trees<F>(map: &Map, handler: F) -> usize
where
    F: Fn(&mut Router),
{
    let mut answer = 0;
    let mut router = Router::new(map);
    while let Some(cell) = router.current() {
        if matches!(cell, Cell::Tree) {
            answer += 1;
        }
        handler(&mut router);
    }
    answer
}

fn main() {
    let map = fs::read_input(INPUT_FILE).unwrap();
    let width = map.find('\n').unwrap();

    let data = map
        .chars()
        .filter_map(|c| match c {
            '.' => Some(Cell::Space),
            '#' => Some(Cell::Tree),
            _ => None,
        })
        .collect::<Vec<_>>();

    let map = Map { width, data };

    let answer1 = count_trees(&map, |router| {
        router.left(1);
        router.down(3);
    });
    println!("answer1: {:?}", answer1);

    let mut answer2 = count_trees(&map, |router| {
        router.left(1);
        router.down(1);
    });

    answer2 *= count_trees(&map, |router| {
        router.left(1);
        router.down(5);
    });

    answer2 *= count_trees(&map, |router| {
        router.left(1);
        router.down(7);
    });

    answer2 *= count_trees(&map, |router| {
        router.left(2);
        router.down(1);
    });

    println!("answer2: {:?}", answer1 * answer2);
}

use crate::io::read_contents;

struct ElfPair(Assignment, Assignment);

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    let pairs = parse_pairs();

    let num_fully_contained_pairs: u32 = pairs
        .iter()
        .map(|p| p.fully_contains_another() as u32)
        .sum();

    println!("Part one: {num_fully_contained_pairs}");
}

fn part_two() {
    let pairs = parse_pairs();
    let num_overlaps: u32 = pairs.iter().map(|p| p.overlaps() as u32).sum();
    println!("Part two: {num_overlaps}");
}

#[derive(Default)]
struct Assignment {
    lower: u16,
    upper: u16,
}

impl ElfPair {
    fn fully_contains_another(&self) -> bool {
        let (first, second) = if self.0.lower < self.1.lower
            || self.0.lower == self.1.lower && self.0.upper > self.1.upper
        {
            (&self.0, &self.1)
        } else {
            (&self.1, &self.0)
        };

        first.lower <= second.lower && first.upper >= second.upper
    }

    fn overlaps(&self) -> bool {
        let (first, second) = if self.0.lower < self.1.lower {
            (&self.0, &self.1)
        } else if self.0.lower == self.1.lower {
            if self.0.upper <= self.1.upper {
                (&self.0, &self.1)
            } else {
                (&self.1, &self.0)
            }
        } else {
            (&self.1, &self.0)
        };

        !(first.upper < second.lower)
    }
}

const INPUT_PATH: &str = "inputs/d4.txt";

fn parse_pairs() -> Vec<ElfPair> {
    let input = read_contents(INPUT_PATH).expect("Should be readable");

    let mut pairs = Vec::new();
    for pair_line in input.lines() {
        let assignments = pair_line.split(",");

        let mut elves = Vec::new();

        for (i, assignment_line) in assignments.enumerate() {
            assert!(i < 2);

            let mut bounds = assignment_line.split("-");

            let lower = bounds
                .next()
                .expect("Should have a lower bound")
                .parse()
                .expect("Should be parseable");
            let upper = bounds
                .next()
                .expect("Should have an upper bound")
                .parse()
                .expect("Should be parseable");

            elves.push(Assignment { lower, upper });
        }

        pairs.push(ElfPair(elves.swap_remove(0), elves.remove(0)));
    }

    pairs
}

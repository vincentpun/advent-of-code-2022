use std::collections::HashSet;

use crate::io::read_contents;

const INPUT_PATH: &str = "inputs/d3.txt";

pub fn run() {
    // Part one.
    part_one();

    // Part two.
    part_two();
}

fn part_one() {
    let rucksacks = parse_rucksack();

    let mut total_score = 0;
    for rucksack in rucksacks {
        let intersection = rucksack
            .left_compartment
            .intersection(&rucksack.right_compartment);
        let score: i32 = intersection.map(|c| c.get_priority()).sum::<usize>() as i32;
        total_score += score;
    }

    println!("Part one: {total_score}");
}

const ALL_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn part_two() {
    let rucksacks = parse_rucksack();

    let score: i32 = rucksacks
        .chunks(3)
        .map(|rucksack_chunk| {
            let mut common_items = ALL_LETTERS.chars().collect::<HashSet<_>>();
            for rucksack in rucksack_chunk {
                let rucksack_items = rucksack.all().map(|c| *c).collect::<HashSet<char>>();
                common_items = common_items
                    .intersection(&rucksack_items)
                    .cloned()
                    .collect();
            }

            common_items
                .into_iter()
                .fold(0, |accum, c| accum + c.get_priority())
        })
        .sum::<usize>() as i32;

    println!("Part two: {score}")
}

#[derive(Default)]
struct Rucksack {
    left_compartment: HashSet<char>,
    right_compartment: HashSet<char>,
}

impl Rucksack {
    fn all(&self) -> impl Iterator<Item = &char> {
        self.left_compartment.union(&self.right_compartment)
    }
}

trait PriorityExt {
    fn get_priority(&self) -> usize;
}

impl PriorityExt for char {
    fn get_priority(&self) -> usize {
        if self.is_ascii_uppercase() {
            usize::try_from(*self as u32 - 'A' as u32 + 27).expect("Should be convertible")
        } else if self.is_ascii_lowercase() {
            usize::try_from(*self as u32 - 'a' as u32 + 1).expect("Should be convertible")
        } else {
            panic!("Illegal!")
        }
    }
}

fn parse_rucksack() -> Vec<Rucksack> {
    let input = read_contents(INPUT_PATH).expect("Should be able to read input");
    let mut rucksacks = Vec::new();

    for line in input.lines() {
        let length = line.len();

        let mut rucksack = Rucksack::default();
        for (i, c) in line.chars().enumerate() {
            if i < length / 2 {
                rucksack.left_compartment.insert(c);
            } else {
                rucksack.right_compartment.insert(c);
            }
        }

        rucksacks.push(rucksack);
    }

    rucksacks
}

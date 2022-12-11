use std::collections::HashSet;

use crate::io::read_contents;

pub fn run() {}

fn part_one() {
    let motions = parse_motion();

    let visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head_coordinates = (0, 0);
    let mut tail_coordinates = (0, 0);

    for motion in motions {}
}

fn perform_motion(
    mut head_coordinates: (i32, i32),
    mut tail_coordinates: (i32, i32),
    motion: Motion
) -> ((i32, i32), (i32, i32)) {
    
}

struct Motion {
    direction: MovementDirection,
    times: u16,
}

enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for MovementDirection {
    fn from(c: char) -> Self {
        match c {
            'U' => MovementDirection::Up,
            'D' => MovementDirection::Down,
            'L' => MovementDirection::Left,
            'R' => MovementDirection::Right,
            _ => panic!(),
        }
    }
}

const INPUT_PATH: &str = "inputs/d9.txt";

fn parse_motion() -> Vec<Motion> {
    let input = read_contents(INPUT_PATH).expect("Input should be readable");

    input
        .lines()
        .map(|line| {
            let components = line.split(" ").collect::<Vec<_>>();

            let direction = components[0].chars().next().unwrap();
            let times = components[1].parse::<u16>().unwrap();

            Motion {
                direction: direction.into(),
                times,
            }
        })
        .collect()
}

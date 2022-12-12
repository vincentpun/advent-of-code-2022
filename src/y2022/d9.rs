use std::collections::HashSet;

use crate::io::read_contents;

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    let motions = parse_motion();

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut head_coordinates = (0, 0);
    let mut tail_coordinates = (0, 0);

    for motion in motions {
        perform_motion(
            &mut head_coordinates,
            &mut tail_coordinates,
            &mut visited,
            motion,
        );
    }

    println!(
        "Number of unique positions visited by tail: {}",
        visited.len()
    );
}

fn part_two() {
    let motions = parse_motion();

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut coordinates: [(i32, i32); 10] = [(0, 0); 10];

    for motion in motions {
        perform_motion_multi(&mut coordinates, &mut visited, motion);
    }

    println!(
        "Number of unique positions visited by tail: {}",
        visited.len()
    )
}

fn perform_motion(
    head_coordinates: &mut (i32, i32),
    tail_coordinates: &mut (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    motion: Motion,
) {
    for _ in 0..motion.times {
        match motion.direction {
            MovementDirection::Up => head_coordinates.1 += 1,
            MovementDirection::Down => head_coordinates.1 -= 1,
            MovementDirection::Left => head_coordinates.0 -= 1,
            MovementDirection::Right => head_coordinates.0 += 1,
        }

        let (_, new_tail_coordinates) =
            catch_up_if_needed(head_coordinates.clone(), tail_coordinates.clone());
        tail_coordinates.0 = new_tail_coordinates.0;
        tail_coordinates.1 = new_tail_coordinates.1;

        visited.insert((tail_coordinates.0, tail_coordinates.1));
    }
}

fn perform_motion_multi(
    coordinates: &mut [(i32, i32); 10],
    visited: &mut HashSet<(i32, i32)>,
    motion: Motion,
) {
    for _ in 0..motion.times {
        match motion.direction {
            MovementDirection::Left => coordinates[0].0 -= 1,
            MovementDirection::Right => coordinates[0].0 += 1,
            MovementDirection::Up => coordinates[0].1 += 1,
            MovementDirection::Down => coordinates[0].1 -= 1,
        }

        for i in 1..10 {
            let (_, new_back_coordinates) =
                catch_up_if_needed(coordinates[i - 1].clone(), coordinates[i].clone());
            coordinates[i].0 = new_back_coordinates.0;
            coordinates[i].1 = new_back_coordinates.1;
        }

        visited.insert((coordinates[9].0, coordinates[9].1));
    }

    // debug(&coordinates);
}

fn catch_up_if_needed(
    front_knot: (i32, i32),
    mut back_knot: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    // Skip if the back does not need to be moved.
    let x_dist = front_knot.0 - back_knot.0;
    let y_dist = front_knot.1 - back_knot.1;
    let back_needs_movement = !(matches!(
        (x_dist.abs(), y_dist.abs()),
        (0, 0) | (1, 0) | (0, 1) | (1, 1)
    ));
    if !back_needs_movement {
        return (front_knot, back_knot);
    }

    match (x_dist, y_dist) {
        (2, 0) => back_knot.0 += 1,
        (-2, 0) => back_knot.0 -= 1,
        (0, 2) => back_knot.1 += 1,
        (0, -2) => back_knot.1 -= 1,

        // Diagonal cases
        (x_dist @ (2 | 1 | -1 | -2), y_dist @ (2 | 1 | -1 | -2)) => {
            back_knot.0 += if x_dist > 0 { 1 } else { -1 };
            back_knot.1 += if y_dist > 0 { 1 } else { -1 };
        }

        (x_dist, y_dist) => panic!("The back shouldn't be this far: {x_dist}, {y_dist}"),
    };

    (front_knot, back_knot)
}

fn debug(coordinates: &[(i32, i32); 10]) {
    let max_x = coordinates.iter().map(|c| c.0).max().unwrap();
    let min_x = coordinates.iter().map(|c| c.0).min().unwrap();

    let max_y = coordinates.iter().map(|c| c.1).max().unwrap();
    let min_y = coordinates.iter().map(|c| c.1).min().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let mut found_match = false;
            for i in 0..8 {
                if coordinates[i] == (x, y) {
                    print!("{}", i + 1);
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                print!(" ");
            }
            print!(" ");
        }
        println!();
    }
}

struct Motion {
    direction: MovementDirection,
    times: u16,
}

#[derive(Debug)]
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

use crate::io::read_contents;

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    let (mut stacks, operations) = parse_operations();
    let crate_mover = CrateMover9000 {};

    for operation in operations {
        crate_mover.perform_move(&mut stacks, operation);
    }

    println!("Part one: Top of each stack:");
    for (i, stack) in stacks.iter().enumerate() {
        println!("Stack {}: {} ", i, stack.last().map(|c| *c).unwrap_or(' '));
    }
}

fn part_two() {
    let (mut stacks, operations) = parse_operations();
    let crate_mover = CrateMover9001 {};

    for operation in operations {
        crate_mover.perform_move(&mut stacks, operation);
    }

    println!("Part two: Top of each stack:");
    for (i, stack) in stacks.iter().enumerate() {
        println!("Stack {}: {} ", i, stack.last().map(|c| *c).unwrap_or(' '));
    }
}

struct MoveOperation {
    times: u8,
    from_stack: usize,
    to_stack: usize,
}

type Stacks = Vec<Vec<char>>;

struct CrateMover9000 {}

impl CrateMover9000 {
    fn perform_move(&self, stacks: &mut Stacks, operation: MoveOperation) {
        for _ in 0..operation.times {
            let item = {
                let from_stack = &mut stacks[operation.from_stack];
                from_stack
                    .pop()
                    .expect("There should be an item in the stack.")
            };

            let to_stack = &mut stacks[operation.to_stack];
            to_stack.push(item);
        }
    }
}

struct CrateMover9001 {}

impl CrateMover9001 {
    fn perform_move(&self, stacks: &mut Stacks, operation: MoveOperation) {
        let mut moved_items = Vec::new();

        let from_stack = &mut stacks[operation.from_stack];
        for _ in 0..operation.times {
            moved_items.push(
                from_stack
                    .pop()
                    .expect("There should be an item in the stack."),
            );
        }

        moved_items.reverse();

        let to_stack = &mut stacks[operation.to_stack];
        for item in moved_items {
            to_stack.push(item);
        }
    }
}

const INPUT_PATH: &str = "inputs/d5.txt";

fn parse_operations() -> (Stacks, Vec<MoveOperation>) {
    let input = read_contents(INPUT_PATH).expect("Input should be readable.");

    // Get the number of stacks from line length.
    let num_stacks = (input
        .lines()
        .next()
        .expect("File should contain contents")
        .len()
        + 1)
        / 4;

    let mut stacks: Stacks = std::iter::repeat(Vec::new()).take(num_stacks).collect();

    let mut lines_iter = input.lines();

    // Start by processing the stack.
    for line in lines_iter.by_ref() {
        // Reached the end. Start processing the move operations.
        if line.is_empty() {
            break;
        }

        // For each line, find the letters in the corresponding columns. If there is a letter, that
        // means it's in the stack.
        for i in 0..num_stacks {
            let c = line
                .chars()
                .nth(i * 4 + 1)
                .expect("Should have character at index");

            if c.is_alphabetic() {
                stacks[i].push(c);
            }
        }
    }

    // We inserted the items from top to bottom. Reverse each "stack" so they actually represent
    // the world.
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let mut move_operations = Vec::new();
    // Process the move operations.
    for line in lines_iter {
        let tokens = line.split(" ");

        let (mut times, mut from_stack, mut to_stack) = (0, 0, 0);
        for (i, token) in tokens.enumerate() {
            // Skip the indices that are not words.
            match i {
                1 => {
                    times = token.parse().expect("Number should be parseable");
                }
                3 => {
                    from_stack = token.parse::<usize>().expect("Number should be parseable") - 1;
                }
                5 => {
                    to_stack = token.parse::<usize>().expect("Number should be parseable") - 1;
                }
                _ => {}
            }
        }

        move_operations.push(MoveOperation {
            times,
            from_stack,
            to_stack,
        });
    }

    (stacks, move_operations)
}

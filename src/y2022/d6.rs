use std::collections::HashSet;
use std::{fs::File, io::BufReader};
use utf8_chars::BufReadCharsExt;

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    println!(
        "Part one: It takes {} characters to be processed before the first start-of-packet.",
        first_marker_index(4) + 1
    );
}

fn part_two() {
    println!(
        "Part two: It takes {} characters to be processed before the first start-of-message.",
        first_marker_index(14) + 1
    );
}

const INPUT_PATH: &str = "inputs/d6.txt";

fn first_marker_index(buffer_size: usize) -> usize {
    let file = File::open(INPUT_PATH).expect("File should be readable");
    let mut file = BufReader::new(file);
    let mut chars = file.chars();

    // Initialize with the first four characters.
    let mut buffer = chars
        .by_ref()
        .take(buffer_size)
        .collect::<Result<Vec<_>, _>>()
        .expect(&format!(
            "There should be at least {} characters",
            buffer_size
        ));
    let mut i = buffer_size - 1;

    loop {
        let hash_set = buffer.iter().collect::<HashSet<_>>();

        if hash_set.len() == buffer_size {
            break;
        }

        buffer.remove(0);

        if let Some(c) = chars.next() {
            buffer.push(c.expect("Should be able to read character"));
            i += 1;
        } else {
            break;
        }
    }

    i
}

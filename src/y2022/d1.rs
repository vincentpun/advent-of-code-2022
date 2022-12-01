use crate::io::read_contents;

const INPUT_PATH: &str = "inputs/d1.txt";

pub fn run() {
    let elf_inventories = parse_inventory_input();

    // Part 1.
    let max_calories = elf_inventories
        .iter()
        .map(|inventory| inventory.total_calories())
        .max()
        .expect("There should be at least one inventory");
    println!("Part 1. Max calories = {max_calories}");

    // Part 2.
    let mut elf_calories = elf_inventories
        .iter()
        .map(|inventory| inventory.total_calories())
        .collect::<Vec<_>>();
    elf_calories.sort();
    elf_calories.reverse();
    let top_three_elves = &elf_calories[0..3];
    assert!(top_three_elves.len() == 3);
    let top_three_elf_calories: i32 = top_three_elves.iter().sum();
    println!("Part 2. Total calories of top three elves = {top_three_elf_calories}");
}

struct Inventory {
    food_calories: Vec<i32>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            food_calories: Vec::new(),
        }
    }

    fn add_food(&mut self, calories: i32) {
        self.food_calories.push(calories)
    }

    fn total_calories(&self) -> i32 {
        self.food_calories.iter().sum()
    }
}

fn parse_inventory_input() -> Vec<Inventory> {
    let mut elf_inventories = Vec::new();
    let inventory_contents = read_contents(&INPUT_PATH).expect("Should be able to read inputs");

    let mut elf_inventory = Inventory::new();
    for line in inventory_contents.lines() {
        if line.is_empty() {
            elf_inventories.push(elf_inventory);
            elf_inventory = Inventory::new();
            continue;
        }

        let food_calories = line.parse::<i32>().expect("Input should be a number");
        elf_inventory.add_food(food_calories);
    }

    elf_inventories
}

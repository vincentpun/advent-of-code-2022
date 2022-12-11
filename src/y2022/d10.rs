use crate::io::read_contents;
use std::collections::VecDeque;

pub fn run() {
    part_one();
}

fn part_one() {
    let commands = parse_commands();

    let mut machine = Machine::new();
    machine.add_commands(&commands);

    let mut signal_strength_sum = 0;

    while !machine.is_idle() {
        println!("Cycle {}, value {}", machine.cycle(), machine.register());
        machine.tick(|cycle, register| match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                signal_strength_sum += cycle as i32 * register;
            }
            _ => {}
        });
    }

    println!("Part one: {signal_strength_sum}");
}

struct Machine {
    cycle: u16,
    register: i32,
    instructions_to_be_executed: VecDeque<Command>,

    executing_instruction: Option<Command>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            cycle: 0,
            register: 1,
            instructions_to_be_executed: VecDeque::new(),
            executing_instruction: None,
        }
    }

    fn cycle(&self) -> u16 {
        self.cycle
    }

    fn register(&self) -> i32 {
        self.register
    }

    fn tick<C>(&mut self, mut callback: C)
    where
        C: FnMut(u16, i32) -> (),
    {
        if self.is_idle() {
            return;
        }

        self.cycle += 1;

        callback(self.cycle, self.register);

        match self.executing_instruction.as_ref() {
            None => match self.instructions_to_be_executed.pop_front() {
                Some(Command::NoOp) | None => {}
                v => self.executing_instruction = v,
            },
            Some(executing_instruction) => match executing_instruction {
                Command::NoOp => panic!(),
                Command::Add(value) => {
                    self.register += *value;
                    self.executing_instruction = None;
                }
            },
        }
    }

    fn is_idle(&self) -> bool {
        self.instructions_to_be_executed.is_empty() && self.executing_instruction.is_none()
    }

    fn add_commands(&mut self, commands: &Vec<Command>) {
        let mut commands = VecDeque::from_iter(commands.iter().cloned());
        self.instructions_to_be_executed.append(&mut commands);
    }
}

#[derive(Clone)]
enum Command {
    NoOp,
    Add(i32),
}

const INPUT_PATH: &str = "inputs/d10.txt";

fn parse_commands() -> Vec<Command> {
    let input = read_contents(INPUT_PATH).expect("Input shuold be readable");

    let mut commands = Vec::new();

    for line in input.lines() {
        let mut splits = line.split(" ");

        let command = match splits.next().unwrap() {
            "noop" => Command::NoOp,
            "addx" => Command::Add(splits.next().unwrap().parse().unwrap()),
            _ => panic!(),
        };
        commands.push(command);
    }

    commands
}

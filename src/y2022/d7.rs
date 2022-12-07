use petgraph::graph::NodeIndex;
use petgraph::visit::DfsPostOrder;
use petgraph::{Direction, Graph};

use crate::io::read_contents;

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    let filesystem = parse_filesystem();
    
}

fn part_two() {}

const INPUT_PATH: &str = "inputs/d7.txt";

enum Node {
    Directory { name: String },
    File { name: String, size: u32 },
}

fn parse_filesystem() -> (Graph<Node, i32>, NodeIndex) {
    let contents = read_contents(INPUT_PATH).expect("File should be readable.");

    let mut filesystem = Graph::new();
    let root_index = filesystem.add_node(Node::Directory {
        name: "/".to_string(),
    });
    let mut cwd = root_index;
    let mut lines = contents.lines();

    let mut line_buffer = Vec::new();
    loop {
        let line = lines.next();
        match line {
            Some(line) => {
                // If it is a command, go parse all the previous output first.
                if line.starts_with("$ ") {
                    filesystem = parse_ls_output(&line_buffer, &cwd, filesystem);

                    let command = line.strip_prefix("$ ").unwrap();
                    // If the command is cd, we'll create a new node in the filesystem, and change
                    // the cwd to it.
                    if let Some(dir) = command.strip_prefix("cd ") {
                        match dir {
                            ".." => {
                                cwd = filesystem
                                    .neighbors_directed(cwd, Direction::Incoming)
                                    .next()
                                    .expect("There should be a parent dir (unless you are in /)");
                            }
                            "/" => {
                                cwd = root_index;
                            }
                            dir => {
                                let children =
                                    filesystem.neighbors_directed(cwd, Direction::Outgoing);
                                let child_dir = children
                                    .filter(|child| {
                                        matches!(
                                            filesystem.node_weight(*child).unwrap(),
                                            Node::File { name, size: _ } if name == dir
                                        )
                                    })
                                    .next()
                                    .unwrap();
                                cwd = child_dir;
                            }
                        }
                    }

                    line_buffer = Vec::new();
                } else {
                    // Otherwise, keep consuming the lines into the buffer.
                    line_buffer.push(line);
                }
            }
            None => {
                filesystem = parse_ls_output(&line_buffer, &cwd, filesystem);
                break;
            }
        }
    }

    (filesystem, root_index)
}

fn parse_ls_output(
    outputs: &Vec<&str>,
    cwd: &NodeIndex,
    mut filesystem: Graph<Node, i32>,
) -> Graph<Node, i32> {
    for line in outputs {
        let components = line.split(" ").collect::<Vec<_>>();
        if components.len() != 2 {
            panic!("There should be two items in a line");
        }

        match components[0] {
            "dir" => {
                let dir_name = components[1];
                let node_index = filesystem.add_node(Node::Directory {
                    name: dir_name.to_string(),
                });
                filesystem.add_edge(*cwd, node_index, 0);
            }
            size => {
                let size = size.parse::<u32>().expect("Size should be parseable");
                let filename = components[1];
                let node_index = filesystem.add_node(Node::File {
                    name: filename.to_string(),
                    size,
                });
                filesystem.add_edge(*cwd, node_index, 0);
            }
        }
    }

    filesystem
}

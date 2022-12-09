use std::collections::HashSet;

use crate::io::read_contents;

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    let mut trees = parse_grid();

    trees = determine_visibility(trees, VisibilityDirection::Left);
    trees = determine_visibility(trees, VisibilityDirection::Right);
    trees = determine_visibility(trees, VisibilityDirection::Top);
    trees = determine_visibility(trees, VisibilityDirection::Bottom);

    let num_trees: i32 = trees
        .iter()
        .map(|tree_row| tree_row.iter())
        .flatten()
        .map(|tree| (tree.visibility.len() > 0) as i32)
        .sum();

    println!("Part one: Number of trees visible from outside the grid: {num_trees}");
}

fn determine_visibility(
    mut trees: Vec<Vec<Tree>>,
    direction: VisibilityDirection,
) -> Vec<Vec<Tree>> {
    if direction == VisibilityDirection::Left || direction == VisibilityDirection::Right {
        for tree_row in trees.iter_mut() {
            let iter: Box<dyn Iterator<Item = &mut Tree>>;
            if direction == VisibilityDirection::Left {
                iter = Box::new(tree_row.iter_mut());
            } else {
                iter = Box::new(tree_row.iter_mut().rev());
            }

            let mut running_max_height = -1;
            for tree in iter {
                if (tree.height) as i32 > running_max_height {
                    running_max_height = tree.height as i32;
                    tree.visibility.insert(direction);
                }
            }
        }
    } else {
        let num_tree_rows = trees.len();
        let num_tree_cols = trees[0].len();

        for tree_col_i in 0..num_tree_cols {
            let mut running_max_height = -1;

            let range_iter: Box<dyn Iterator<Item = usize>> =
                if direction == VisibilityDirection::Top {
                    Box::new((0..num_tree_rows).into_iter())
                } else {
                    Box::new((0..num_tree_rows).rev())
                };

            for tree_row_i in range_iter {
                let tree = &mut trees[tree_row_i][tree_col_i];

                if (tree.height) as i32 > running_max_height {
                    running_max_height = tree.height as i32;
                    tree.visibility.insert(direction);
                }
            }
        }
    }

    trees
}

fn part_two() {
    let trees = parse_grid();

    let highest_scenic_score = trees
        .iter()
        .enumerate()
        .map(|(i, tree_row)| {
            tree_row.iter().enumerate().map({
                let trees = &trees;
                move |(j, _)| {
                    visible_trees_from_tree(&trees, (i.clone(), j), VisibilityDirection::Top)
                        * visible_trees_from_tree(
                            &trees,
                            (i.clone(), j),
                            VisibilityDirection::Bottom,
                        )
                        * visible_trees_from_tree(&trees, (i.clone(), j), VisibilityDirection::Left)
                        * visible_trees_from_tree(
                            &trees,
                            (i.clone(), j),
                            VisibilityDirection::Right,
                        )
                }
            })
        })
        .flatten()
        .max()
        .unwrap();

    println!("Part two: Highest scenic score = {highest_scenic_score}");
}

fn visible_trees_from_tree(
    trees: &Vec<Vec<Tree>>,
    tree_idx: (usize, usize),
    direction: VisibilityDirection,
) -> u32 {
    let (mut i, mut j) = tree_idx;

    let tree = &trees[i][j];
    let tree_height_threshold = tree.height;
    let mut viewing_distance = 0;

    loop {
        match direction {
            VisibilityDirection::Top => {
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            VisibilityDirection::Bottom => {
                if j == trees[0].len() - 1 {
                    break;
                }
                j += 1;
            }
            VisibilityDirection::Left => {
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            VisibilityDirection::Right => {
                if i == trees.len() - 1 {
                    break;
                }
                i += 1;
            }
        }

        viewing_distance += 1;

        if &trees[i][j].height >= &tree_height_threshold {
            break;
        }
    }

    viewing_distance
}

struct Tree {
    height: u32,
    visibility: HashSet<VisibilityDirection>,
}

impl Tree {
    fn new(height: u32) -> Tree {
        Tree {
            height,
            visibility: HashSet::new(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum VisibilityDirection {
    Top,
    Bottom,
    Left,
    Right,
}

const INPUT_PATH: &str = "inputs/d8.txt";

fn parse_grid() -> Vec<Vec<Tree>> {
    let contents = read_contents(INPUT_PATH).expect("Should be readable");
    let mut trees = Vec::new();

    for line in contents.lines() {
        trees.push(Vec::new());
        let tree_row = trees.last_mut().unwrap();
        for c in line.chars() {
            let tree_height = c.to_digit(10).unwrap();
            let tree = Tree::new(tree_height);
            tree_row.push(tree);
        }
    }

    trees
}

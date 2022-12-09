use std::fs::File;
use std::io::{BufReader, BufRead};

use id_tree::*;
use id_tree::InsertBehavior::*;

#[derive(Debug, Default)]
struct Item {
    size: usize
}

fn total_size(tree: &Tree<Item>, node: &Node<Item>) -> usize {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(&child).unwrap());
    }
    total
}

fn main() {
    let file = File::open("./data/day_07.txt").expect("Could not find file");
    let lines = BufReader::new(file).lines();

    let mut directory_tree = Tree::new();
    let root = directory_tree.insert(
        Node::new( Item {
            size: 0,
        }),
        AsRoot,
    ).unwrap();
    let mut curr = root;

    for line in lines {
        let mut line = line.expect("Could not read line!");

        if line.starts_with("$ ") {
            if line == "$ ls" {
                // Not really anything to do hear...
                // We know that any non-$ lines are output from ls
                continue;
            }

            let path = line.split_off(5);
            match path.as_str() {
                "/" => {
                    // Do nothing
                },
                ".." => {
                    curr = directory_tree
                        .get(&curr).unwrap()
                        .parent().unwrap()
                        .clone();
                },
                _ => {
                    let node = Node::new( Item {
                        size: 0
                    });
                    curr = directory_tree.insert(node, UnderNode(&curr)).unwrap();
                }
            };
        } else {
            if line.starts_with("dir") {
                // Do nothing
            } else {
                let (size, _) = line.split_once(' ').unwrap();
                let node = Node::new( Item {
                    size: size.parse().unwrap()
                });
                directory_tree.insert(node, UnderNode(&curr)).unwrap();
            }
        }
    }

    // let sum = directory_tree
    //     .traverse_pre_order(directory_tree.root_node_id().unwrap()).unwrap()
    //     .filter(|node| node.children().len() != 0)
    //     .map(|node| total_size(&directory_tree, node) as u64)
    //     .filter(|&s| s <= 100_000)
    //     .sum::<u64>();

    let used_space = total_size(
        &directory_tree,
        directory_tree.get(directory_tree.root_node_id().unwrap()).unwrap()
    );
    let unused_space  = 70_000_000 - used_space;
    let space_to_free = 30_000_000 - unused_space;

    let min = directory_tree
        .traverse_pre_order(directory_tree.root_node_id().unwrap()).unwrap()
        .filter(|node| node.children().len() != 0)
        .map(|node| total_size(&directory_tree, node) as u64)
        .filter(|&s| s >= space_to_free as u64)
        .min().unwrap();

    println!("Smallest directory to delete: {}", min);
}
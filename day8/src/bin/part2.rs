use aoc::*;
use day8::*;

fn main() {
    let data: Vec<usize> = parser::input::<String>()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let tree = Node::parse(&data).0;
    let value = tree.value();

    answer!("The value of the root node is {}.", value);
}

use aoc::*;
use day8::*;

fn main() {
    let data: Vec<usize> = parser::input::<String>()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let tree = Node::parse(&data).0;
    let sum = tree.sum();

    answer!("The sum of all metadata entries is {}.", sum);
}

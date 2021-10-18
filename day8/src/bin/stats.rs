use aoc::*;
use day8::*;

fn main() {
    let data: Vec<usize> = parser::input::<String>()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let tree = Node::parse(&data).0;
    let depth = tree.depth();
    let size = tree.size();

    answer!("Your tree has a depth of {}.", depth);
    answer!("It contains {} nodes.", size);
}

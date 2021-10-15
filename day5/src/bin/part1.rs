use aoc::*;
use day5::*;

fn main() {
    let polymer: String = parser::chars::<char>().collect();
    let polymer = react(polymer);

    answer!(
        "After fully reacting the polymer, {} units remains.",
        polymer.len()
    );
}

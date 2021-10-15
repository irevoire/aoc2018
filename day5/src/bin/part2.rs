use aoc::*;
use day5::*;

fn main() {
    let polymer: String = parser::chars::<char>().collect();
    let smolest_size = ('a'..='z')
        .map(|unit| {
            let polymer = polymer.replace(unit, "");
            let polymer = polymer.replace(unit.to_ascii_uppercase(), "");

            react(polymer).len()
        })
        .min()
        .unwrap();

    println!(
        "The length of the shortest polymer you can produce is {}.",
        smolest_size
    );
}

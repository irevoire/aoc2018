use std::collections::HashMap;

use aoc::parser;

fn sub_checksum(input: &str) -> (bool, bool) {
    let map = input
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut map, element| {
            *map.entry(element).or_default() += 1;
            map
        });

    (
        map.values().any(|&count| count == 2),
        map.values().any(|&count| count == 3),
    )
}

fn main() {
    let checksum = parser::lines_from_args(1)
        .map(|line| sub_checksum(&line))
        .fold((0, 0), |checksum, (twos, threes)| {
            (checksum.0 + twos as usize, checksum.1 + threes as usize)
        });
    println!(
        "The checksum of your list of box ids is {}",
        checksum.0 * checksum.1
    );
}
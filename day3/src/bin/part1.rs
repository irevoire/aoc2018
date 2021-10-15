use aoc::*;
use day3::Claim;
use std::collections::HashMap;

fn main() {
    let result = parser::lines::<Claim>()
        .flat_map(|claim| claim.start.to(claim.end).unwrap())
        .fold(HashMap::new(), |mut map, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        })
        .into_values()
        .filter(|&key| key > 1)
        .count();

    answer!(
        "There is {} square inches of fabric that are within two or more claims",
        result
    );
}

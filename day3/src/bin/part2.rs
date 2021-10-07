use day3::Claim;
use std::collections::HashMap;

fn main() {
    let input = aoc::parser::lines_from_args_as::<Claim>(1).collect::<Vec<_>>();

    let map = input
        .iter()
        .flat_map(|claim| claim.start.to(claim.end).unwrap())
        .fold(HashMap::new(), |mut map, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        });

    let result = input
        .iter()
        .find(|claim| {
            claim
                .start
                .to(claim.end)
                .unwrap()
                .all(|point| map[&point] == 1)
        })
        .unwrap()
        .id;

    println!(
        "There is {} square inches of fabric that are within two or more claims",
        result
    );
}

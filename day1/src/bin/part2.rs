use std::collections::HashSet;

fn main() {
    let input = aoc::parser::lines().collect::<Vec<i32>>();
    let result = input
        .iter()
        .cycle()
        .scan(0, |acc, el| {
            *acc += el;
            Some(*acc)
        })
        .scan(HashSet::new(), |acc, el| Some((!acc.insert(el), el)))
        .find(|(condition, _)| *condition)
        .unwrap();

    println!("Frequency: {}", result.1);
}

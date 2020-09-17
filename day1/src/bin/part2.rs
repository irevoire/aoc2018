use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    let input = aoc::parser::lines_from_args(1).collect::<Vec<String>>();
    let iter = input
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .cycle()
        .scan(0, |acc, el| {
            *acc += el;
            Some(*acc)
        });

    let mut res = 0;

    for el in iter {
        res = el;
        match set.insert(el) {
            false => break,
            true => (),
        }
    }

    println!("result: {}", res);
}

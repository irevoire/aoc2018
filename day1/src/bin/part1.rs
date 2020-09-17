fn main() {
    let res: i32 = aoc::parser::lines_from_args(1)
        .map(|line| line.parse::<i32>().unwrap())
        .sum();

    println!("result: {}", res);
}

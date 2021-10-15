use aoc::*;

fn main() {
    let res: i32 = parser::lines::<i32>().sum();
    answer!("result: {}", res);
}

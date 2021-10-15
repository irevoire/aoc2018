use aoc::*;

fn main() {
    let frequency: i32 = parser::lines::<i32>().sum();
    answer!(
        "After all of the changes in frequency applied, the resulting frequency is {}.",
        frequency
    );
}

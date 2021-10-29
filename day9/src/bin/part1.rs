use aoc::*;
use day9::*;

fn main() {
    let game = parser::input::<Game>();

    answer!("The winning Elf's score is {}.", game.play());
}

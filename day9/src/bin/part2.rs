use aoc::*;
use day9::*;

fn main() {
    let mut game = parser::input::<Game>();
    game.marbles *= 100;

    answer!(
        "With the last marble being 100 times larger, the winning Elf's score is {}.",
        game.play()
    );
}

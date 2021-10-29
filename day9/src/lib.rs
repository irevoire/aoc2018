use aoc::*;

use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Game {
    players: usize,
    pub marbles: usize,
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let relevant_number: Vec<usize> = s
            .split_whitespace()
            .filter_map(|chunk| chunk.parse().ok())
            .collect();
        let players = relevant_number[0];
        let marbles = relevant_number[1];
        Ok(Game { players, marbles })
    }
}

impl Game {
    pub fn play(self) -> usize {
        let mut players: Vec<usize> = vec![0; self.players];
        let mut board: CyclicList<usize> = [0, 1].into_iter().collect();

        let turns = (0..self.players).cycle().take(self.marbles).enumerate();

        for (marble, current_player) in turns {
            if marble % 23 == 0 {
                board.move_left_n(7);
                players[current_player] += marble + board.pop_right().unwrap();
            } else {
                board.move_right_n(2);
                board.push_right(marble);
            }
        }

        *players.iter().max().unwrap()
    }
}

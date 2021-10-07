use std::str::FromStr;

use aoc::Coord;
#[derive(Debug)]
pub struct Claim {
    pub id: usize,
    pub start: Coord<usize>,
    pub end: Coord<usize>,
}

impl FromStr for Claim {
    type Err = aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let claim_parts = s.replace(&['#', '@', ':'][..], "").replace('x', ",");
        let claim_parts: Vec<_> = claim_parts.split(" ").map(str::trim).collect();
        let id = claim_parts[0].parse()?;
        let start = claim_parts[2].parse()?;
        let size = claim_parts[3].parse::<Coord<usize>>()?;
        let end = start + size - 1;

        Ok(Self { id, start, end })
    }
}

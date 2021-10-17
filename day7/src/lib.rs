use aoc::*;
use std::str::FromStr;

pub struct Dependency {
    pub step: char,
    pub depends_on: char,
}

impl FromStr for Dependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = s.split_whitespace().collect();
        Ok(Dependency {
            step: s[1].chars().next().unwrap(),
            depends_on: s[7].chars().next().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct Step {
    pub name: char,
    pub depends_on: Vec<char>,
    pub unlock: Vec<char>,
}

impl Step {
    pub fn new(name: char) -> Self {
        Self {
            name,
            depends_on: Vec::new(),
            unlock: Vec::new(),
        }
    }

    pub fn try_unlock(&mut self, name: char) -> bool {
        self.depends_on.binary_remove(name);
        self.depends_on.is_empty()
    }
}

use aoc::*;

pub struct Node {
    childs: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    pub fn parse(mut data: &[usize]) -> (Self, &[usize]) {
        let mut childs = Vec::new();
        let nb_childs = data[0];
        let nb_metadata = data[1];

        data = &data[2..];

        for _ in 0..nb_childs {
            let (child, new_data) = Self::parse(data);
            data = new_data;
            childs.push(child);
        }

        let metadata = data[..nb_metadata].to_vec();

        (Self { childs, metadata }, &data[nb_metadata..])
    }

    pub fn sum(&self) -> usize {
        self.metadata.iter().sum::<usize>() + self.childs.iter().map(Self::sum).sum::<usize>()
    }
}

fn main() {
    let data: Vec<usize> = parser::input::<String>()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let tree = Node::parse(&data).0;
    let sum = tree.sum();

    answer!("The sum of all metadata entries is {}.", sum);
}

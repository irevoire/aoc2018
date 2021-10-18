pub struct Node {
    pub childs: Vec<Node>,
    pub metadata: Vec<usize>,
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

    pub fn value(&self) -> usize {
        if self.childs.is_empty() {
            self.sum()
        } else {
            self.metadata
                .iter()
                .filter(|value| (1..=self.childs.len()).contains(value))
                .map(|&id| self.childs[(id - 1)].value())
                .sum()
        }
    }
}

use aoc::*;
use day8::*;

trait Plot {
    fn plot(&self, idx: usize) -> usize;
}

impl Plot for Node {
    fn plot(&self, mut idx: usize) -> usize {
        let base = idx;
        println!(
            "\tg{} [xlabel=\"{}\"]",
            base,
            self.metadata
                .iter()
                .map(|m| m.to_string())
                .collect::<String>()
        );
        for child in &self.childs {
            println!("\tg{} -> g{}", base, idx + 1);
            idx = child.plot(idx + 1);
        }

        idx
    }
}

fn main() {
    if atty::is(atty::Stream::Stdout) {
        eprintln!(
            "This binary generate an output compatible with `dot`.
`dot` come from the `graphviz` package.
To use this binary you must pipe its output in the command `dot`.
cargo run --release --bin graph | dot -T svg > graph.svg
You can also generate `gif` or `png`.
"
        );
        return;
    }
    let data: Vec<usize> = parser::input::<String>()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let tree = Node::parse(&data).0;

    println!("digraph G {{");
    println!("\trankdir=\"LR\"");
    tree.plot(0);
    println!("}}");
}

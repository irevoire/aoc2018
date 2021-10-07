use aoc::parser;

/// return the number of letters in common between the two word + a string containing these letters
fn letter_in_common(base: &str, cmp: &str) -> (usize, String) {
    let result = base
        .chars()
        .zip(cmp.chars())
        .filter_map(|(base, cmp)| (base == cmp).then(|| base))
        .collect::<String>();

    (base.len() - result.len(), result)
}

fn main() {
    let box_ids = parser::lines().collect::<Vec<String>>();

    for (idx, base_id) in box_ids.iter().enumerate() {
        for cmp_id in box_ids.iter().skip(idx + 1) {
            let (nb, result) = letter_in_common(base_id, cmp_id);
            if nb == 1 {
                println!("{}", result);
            }
        }
    }
}

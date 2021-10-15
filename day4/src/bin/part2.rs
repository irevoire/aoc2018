use aoc::*;

fn main() {
    let map = day4::parse();

    let (guard, schedule) = map
        .par_iter()
        .max_by_key(|(_id, sleep)| sleep.iter().max().unwrap())
        .unwrap();
    let minute = schedule
        .par_iter()
        .enumerate()
        .max_by_key(|(_idx, &minute)| minute)
        .unwrap()
        .0;

    println!(
        "The guard that is most frequently asleep on the same minute is {}.",
        guard
    );
    println!(
        "The minute he was the most frequently asleep is {}.",
        minute
    );
    answer!("It’s ID multiplied by the minute: {}", guard * minute);
}

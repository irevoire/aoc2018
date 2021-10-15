use aoc::*;

fn main() {
    let map = day4::parse();

    let (guard, schedule) = map
        .par_iter()
        .max_by_key(|(_id, sleep)| sleep.iter().sum::<usize>())
        .unwrap();
    let minute = schedule
        .par_iter()
        .enumerate()
        .max_by_key(|(_idx, &minute)| minute)
        .unwrap()
        .0;

    println!("The guard that has the most minutes asleep is {}.", guard);
    println!("That guard spent the {}th minute asleep the most", minute);
    answer!("It’s ID multiplied by the minute: {}", guard * minute);
}

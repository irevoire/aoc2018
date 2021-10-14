fn main() {
    let map = day4::parse();

    let (guard, schedule) = map
        .iter()
        .max_by_key(|(_id, sleep)| sleep.into_iter().sum::<usize>())
        .unwrap();
    let minute = schedule
        .into_iter()
        .enumerate()
        .max_by_key(|(_idx, &minute)| minute)
        .unwrap()
        .0;

    println!("The guard that has the most minutes asleep is {}.", guard);
    println!("That guard spent the {}th minute asleep the most", minute);
    println!("It’s ID multiplied by the minute: {}", guard * minute);
}

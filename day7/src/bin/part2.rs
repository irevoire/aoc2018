use std::collections::HashMap;

use aoc::*;
use day7::*;

fn main() {
    let mut steps: HashMap<_, _> = ('A'..='Z').map(|c| (c, Step::new(c))).collect();
    for dep in parser::lines::<Dependency>() {
        steps
            .get_mut(&dep.step)
            .unwrap()
            .unlock
            .binary_insert(dep.depends_on);
        steps
            .get_mut(&dep.depends_on)
            .unwrap()
            .depends_on
            .binary_insert(dep.step);
    }

    for step in steps.values_mut() {
        step.depends_on.sort();
        step.unlock.sort();
    }

    let mut available_steps: Vec<_> = steps
        .values()
        .filter(|step| step.depends_on.is_empty())
        .map(|step| step.name)
        .collect();
    available_steps.sort();
    available_steps.iter().for_each(|step| {
        steps.remove(&step);
    });

    // a worker is a tuple of (Step, time remaining)
    let mut workers: Vec<(Option<char>, usize)> = vec![(None, 0); 5];
    let mut step_done = Vec::new();
    let mut time = 0;

    while !available_steps.is_empty() || workers.iter().any(|w| w.0.is_some()) {
        for worker in workers.iter_mut().filter(|w| w.0.is_none()) {
            if !available_steps.is_empty() {
                let step = available_steps.remove(0);
                worker.0 = Some(step);
                worker.1 = 60 + (step as u8 - b'A') as usize;
            }
        }
        for worker in workers.iter_mut().filter(|w| matches!(w, (Some(_), 0))) {
            let done = std::mem::take(&mut worker.0).unwrap();
            step_done.push(done);
            unlock_all_steps(&mut available_steps, done, &mut steps);
        }
        time += 1;
        workers
            .iter_mut()
            .filter(|(s, _)| s.is_some())
            .for_each(|(_, t)| *t -= 1);
    }

    answer!(
        "With 5 workers, it takes {} seconds to complete all the steps.",
        time
    );
}

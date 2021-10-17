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

    let mut step_done = Vec::new();

    while !available_steps.is_empty() {
        let done = available_steps.remove(0);
        step_done.push(done);
        unlock_all_steps(&mut available_steps, done, &mut steps);
    }

    let order = step_done.iter().collect::<String>();

    answer!(
        "The steps in your instructions should be completed in the following order {}.",
        order
    );
}

pub fn unlock_all_steps(
    available_steps: &mut Vec<char>,
    done: char,
    steps: &mut HashMap<char, Step>,
) {
    let keys: Vec<char> = steps.keys().copied().collect();
    for key in keys {
        let step = steps.get_mut(&key).unwrap();

        if step.try_unlock(done) {
            available_steps.binary_insert(key);
            steps.remove(&key);
        }
    }
}

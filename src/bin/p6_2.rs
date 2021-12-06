use std::collections::HashMap;
use aoc::*;

fn main() {
    let num_of_days = 256;
    let new_fish_timer = 8;
    let old_fish_timer_reset = 6;

    if let Ok(mut lines) = read_lines("./input/p6.txt") {
        let line = lines.next().unwrap().unwrap();

        let state = line.split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // println!("Initial state: {}, {:?}", state.len(), state);

        let mut fish = state.iter()
            .fold(HashMap::new(), |mut acc, timer| {
                let entry = acc.entry(*timer).or_insert(0);
                *entry += 1;

                acc
            });

        for _day in 0..num_of_days {
            let mut new_values = (0..new_fish_timer)
                .map(|i| (i, *fish.get(&(i + 1)).unwrap_or(&0)))
                .collect::<HashMap<_, _>>();

            let old_fish_reset = new_values.entry(old_fish_timer_reset).or_insert(0);
            *old_fish_reset += fish.get(&0).unwrap_or(&0);

            let new_fish = new_values.entry(new_fish_timer).or_insert(0);
            *new_fish += fish.get(&0).unwrap_or(&0);

            fish = new_values.clone();
        }

        println!("{}", fish.iter().map(|x| x.1).sum::<i64>());
    }
}
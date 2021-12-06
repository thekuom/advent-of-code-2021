use aoc::*;

fn main() {
    let num_of_days = 80;
    let new_fish_timer = 8;
    let old_fish_timer_reset = 6;

    if let Ok(mut lines) = read_lines("./input/p6.txt") {
        let line = lines.next().unwrap().unwrap();

        let mut state = line.split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // println!("Initial state: {}, {:?}", state.len(), state);

        for _i in 0..num_of_days {
            let mut num_fish_to_add = 0;
            for fish_timer in state.iter_mut() {
                *fish_timer -= 1;
                if *fish_timer < 0 {
                    num_fish_to_add += 1;
                    *fish_timer = old_fish_timer_reset;
                }
            }

            for _j in 0..num_fish_to_add {
                state.push(new_fish_timer);
            }

            // println!("After {} days: {}, {:?}", _i, state.len(), state);
        }

        println!("{}", state.len());
    }
}
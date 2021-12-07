use aoc::*;

fn main() {
    if let Ok(mut lines) = read_lines("./input/p7.txt") {
        let mut positions: Vec<_> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        positions.sort();

        let max = positions.last().unwrap();

        let fuel = (0..=*max).fold(None, |min_fuel, target| {
            let fuel_candidate = positions.iter().fold(0, |acc, pos| {
                let distance = (pos - target).abs();

                acc + ((distance as f64 / 2 as f64) * (1 + distance) as f64).round() as i32
            });

            match (min_fuel, fuel_candidate) {
                (Some(x), y) if x < y => min_fuel,
                _ => Some(fuel_candidate),
            }
        }).unwrap();

        println!("{:?}", fuel);
    }
}

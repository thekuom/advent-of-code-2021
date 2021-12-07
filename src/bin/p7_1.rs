use aoc::*;

fn main() {
    if let Ok(mut lines) = read_lines("./input/p7.txt") {
        let mut positions: Vec<_> = lines.next().unwrap().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        positions.sort();

        println!("{:?}", positions);

        let median = if positions.len() % 2 == 1 {
            positions[(positions.len() + 1) / 2]
        } else {
            positions[positions.len() / 2]
        };

        let fuel = positions.iter().fold(0, |acc, x| {
            acc + (x - median).abs()
        });

        println!("{}", fuel);
    }
}

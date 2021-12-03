use aoc::read_lines;

fn main() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines("./input/p2.txt") {
        for line in lines {
            let line = line.unwrap();
            let mut split = line.split(" ");
            let command = split.next().unwrap();
            let amount = split.next().unwrap().parse::<i32>().unwrap();

            horizontal_position += match command {
                "forward" => amount,
                _ => 0,
            };

            depth += match command {
                "forward" => aim * amount,
                _ => 0,
            };

            aim += match command {
                "down" => amount,
                "up" => -amount,
                _ => 0,
            };

            // println!("command: {}", line);
            // println!("horiz: {}, depth: {}, aim: {}", horizontal_position, depth, aim);
        }
    }

    println!("{}", horizontal_position * depth);
}

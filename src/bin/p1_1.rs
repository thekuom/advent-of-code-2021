use aoc::read_lines;

fn main() {
    let mut result = 0;
    let mut total = 0;
    let mut prev_line: Option<i32> = None;

    if let Ok(lines) = read_lines("./input/p1.txt") {
        for line in lines {
            if line.is_err() {
                continue;
            }

            total += 1;

            if let Ok(parsed_line) = line.unwrap().parse::<i32>() {
                let increase_amt = match prev_line {
                    Some(prev_val) if prev_val < parsed_line => 1,
                    _ => 0
                };

                result += increase_amt;

                prev_line = Some(parsed_line);
            }
        }
    }

    println!("{}/{}", result, total);
}
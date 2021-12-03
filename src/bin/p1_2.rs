use aoc::read_lines;

fn main() {
    let mut result = 0;
    let mut last_four = [None, None, None, None];

    if let Ok(lines) = read_lines("./input/p1.txt") {
        for line in lines {
            if line.is_err() {
                continue;
            }

            for i in 0..3 {
                last_four[i] = last_four[i + 1].clone();
            }

            if let Ok(parsed_line) = line.unwrap().parse::<i32>() {
                last_four[3] = Some(parsed_line);

                if last_four.iter().any(|x| x.is_none()) {
                    continue;
                }

                let prev_window_sum: i32 = last_four.iter().map(|x| x.unwrap()).take(3).sum();
                let current_window_sum: i32 = last_four.iter().map(|x| x.unwrap()).skip(1).take(3).sum();

                if prev_window_sum < current_window_sum {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}

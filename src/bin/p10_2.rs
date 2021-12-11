use std::collections::HashMap;

use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p10.txt") {
        let mut completion_scores = Vec::new();

        let score_table = HashMap::from([
            (')', 1),
            (']', 2),
            ('}', 3),
            ('>', 4),
        ]);

        let bracket_match = HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>'),
        ]);

        'line_loop: for line in lines {
            let line = line.unwrap();
            let mut line_stack = Vec::new();

            for char in line.chars() {
                let invalid_line = match char {
                    x if ['(', '[', '{', '<'].contains(&x) => {
                        line_stack.push(char);
                        false
                    },
                    x if [')', ']', '}', '>'].contains(&x) => match line_stack.get(line_stack.len() - 1) {
                        Some(c) if bracket_match.get(&c).unwrap() == &x => {
                            line_stack.pop();
                            false
                        },
                        _ => true,
                    },
                    _ => true,
                };

                if invalid_line {
                    continue 'line_loop;
                }
            }

            if line_stack.len() > 0 {
                line_stack.reverse();
                completion_scores.push(line_stack.iter().fold(0 as i64, |acc: i64, item|
                    5 * acc + score_table.get(bracket_match.get(item).unwrap()).unwrap()
                ));
            }
        }

        completion_scores.sort();

        println!("{}", completion_scores[completion_scores.len() / 2]);
    }
}

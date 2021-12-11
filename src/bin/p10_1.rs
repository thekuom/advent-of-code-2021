use std::collections::HashMap;

use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p10.txt") {
        let mut score = 0;

        let score_table = HashMap::from([
            (')', 3),
            (']', 57),
            ('}', 1197),
            ('>', 25137),
        ]);

        let bracket_match = HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>'),
        ]);

        for line in lines {
            let line = line.unwrap();
            let mut line_stack = Vec::new();

            for char in line.chars() {
                let score_to_add = match char {
                    x if ['(', '[', '{', '<'].contains(&x) => {
                        line_stack.push(char);
                        0
                    },
                    x if [')', ']', '}', '>'].contains(&x) => match line_stack.get(line_stack.len() - 1) {
                        Some(c) if bracket_match.get(&c).unwrap() == &x => {
                            line_stack.pop();
                            0
                        },
                        _ => *score_table.get(&x).unwrap(),
                    },
                    _ => 0,
                };

                if score_to_add > 0 {
                    score += score_to_add;
                    break;
                }
            }
        }

        println!("{}", score);
    }
}
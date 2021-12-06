use std::cmp::{max, min};
use std::collections::HashMap;
use aoc::*;

fn main() {
    if let Ok(text_lines) = read_lines("./input/p5.txt") {
        let mut lines = Vec::new();
        let mut covered_points = HashMap::<(i32, i32), i32>::new();

        for text_line in text_lines {
            let points = text_line.unwrap().split(" -> ")
                .map(|x| {
                    let split = x
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    (split[0], split[1])
                })
                .collect::<Vec<(i32, i32)>>();

            lines.push(Line {
                from: points[0],
                to: points[1],
            });
        }

        for line in lines {
            for covered_point in line.get_covered_points() {
                let counter = covered_points.entry(covered_point).or_default();
                *counter += 1;
            }
        }

        println!("{}", covered_points.iter().filter(|&x| *x.1 >= 2).count());
    }
}

#[derive(Debug)]
struct Line {
    from: (i32, i32),
    to: (i32, i32),
}

impl Line {
    pub fn get_covered_points(&self) -> Vec<(i32, i32)> {
        let x1 = self.from.0;
        let x2 = self.to.0;
        let y1 = self.from.1;
        let y2 = self.to.1;

        if x2 == x1 {
            return (0..=(y2 - y1).abs())
                .map(|s| (x1, min(y1, y2) + s))
                .collect();
        }

        let m = (y2 - y1) as f64 / (x2 - x1) as f64;
        let b = y1 as f64 - m * x1 as f64;

        (min(x1, x2)..=max(x1,x2))
            .map(|x| (x, (m * x as f64 + b) as i32))
            .collect()
    }
}
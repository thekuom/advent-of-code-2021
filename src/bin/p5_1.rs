use std::cmp::min;
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
            // println!("{:?}", line);
            for covered_point in line.get_covered_points() {
                // println!("{:?}", covered_point);
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
        match (self.from, self.to)  {
            ((x1, y1), (x2, y2)) if x1 == x2 =>
                (0..=(y2 - y1).abs())
                    .map(|s| (x1, min(y1, y2) + s))
                    .collect(),
            ((x1, y1), (x2, y2)) if y1 == y2 =>
                (0..=(x2 - x1).abs())
                    .map(|s| (min(x1, x2) + s, y1))
                    .collect(),
            _ => Vec::new()
        }
    }
}
use std::collections::HashMap;
use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p14.txt") {
        let mut polymer = String::new();
        let mut pair_insertion_rules = HashMap::new();

        for (idx, line) in lines.enumerate() {
            let line = line.unwrap();

            if idx == 0 {
                polymer = line;
                continue;
            }

            if line.len() > 0 {
                let split = line.split(" -> ").collect::<Vec<_>>();
                pair_insertion_rules.insert(split[0].to_string(), split[1].to_string().parse::<char>().unwrap());
            }
        }

        polymer = (0..10).fold(polymer, |prev, _step| {
            let mut new_str = String::new();
            let iter1 = prev.chars();
            let iter2 = prev.chars().skip(1);

            for (i, (a, b)) in iter1.zip(iter2).enumerate() {
                if i == 0 {
                    new_str.push(a);
                }

                if let Some(char_to_insert) = pair_insertion_rules.get(&format!("{}{}", a, b)) {
                    new_str.push(*char_to_insert);
                }

                new_str.push(b);
            }

            new_str
        });

        let by_char = polymer
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                let entry = acc.entry(c).or_insert(0);
                *entry += 1;
                acc
            });
        let mut by_char = by_char
            .iter()
            .collect::<Vec<_>>();
        by_char.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        println!("{}", by_char[by_char.len() - 1].1 - by_char[0].1);
    }
}
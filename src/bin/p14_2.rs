use std::collections::HashMap;
use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p14.txt") {
        let mut pair_insertion_rules = HashMap::new();
        let mut pairs_map = HashMap::new();
        let mut chars_count = HashMap::new();

        for (idx, line) in lines.enumerate() {
            let line = line.unwrap();

            if idx == 0 {
                let iter1 = line.chars();
                let iter2 = line.chars().skip(1);

                pairs_map = iter1.zip(iter2).fold(pairs_map, |mut acc, x| {
                    *acc.entry(x).or_insert(0 as i64) += 1;
                    acc
                });
                chars_count = line
                    .chars()
                    .fold(HashMap::new(), |mut acc, c| {
                        *acc.entry(c).or_insert(0 as i64) += 1;
                        acc
                    });
                continue;
            }

            if line.len() > 0 {
                let split = line.split(" -> ").collect::<Vec<_>>();
                let pair_chars = split[0].chars().collect::<Vec<_>>();
                pair_insertion_rules.insert((pair_chars[0], pair_chars[1]), split[1].to_string().parse::<char>().unwrap());
            }
        }

        for _step in 0..40 {
            let mut new_pairs_map = pairs_map.clone();
            for pair in pairs_map.iter_mut() {
                if let Some(rule) = pair_insertion_rules.get(&pair.0) {
                    *chars_count.entry(*rule).or_insert(0 as i64) += *pair.1;
                    *new_pairs_map.entry((pair.0.0, *rule)).or_insert(0 as i64) += *pair.1;
                    *new_pairs_map.entry((*rule, pair.0.1)).or_insert(0 as i64) += *pair.1;
                    *new_pairs_map.entry((pair.0.0, pair.0.1)).or_insert(0 as i64) -= *pair.1;
                }
            }
            pairs_map = new_pairs_map;
        }

        let mut by_char = chars_count
            .iter()
            .collect::<Vec<_>>();
        by_char.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        println!("{}", by_char[by_char.len() - 1].1 - by_char[0].1);
    }
}

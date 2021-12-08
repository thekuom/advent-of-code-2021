use std::collections::{HashMap, HashSet};
use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p8.txt") {
        let mut result = 0;

        for line in lines {
            let line = line.unwrap();
            let split = line.split(" | ").collect::<Vec<_>>();
            let unique_digits = split[0].split(" ").collect::<Vec<_>>();
            let output_digits = split[1].split(" ").collect::<Vec<_>>();

            let mut digit_segments: HashMap<i32, HashSet<char>> = (0..=9)
                .map(|i| (i, HashSet::new()))
                .collect::<_>();

            let mut segment_wiring = HashMap::new();

            let one = unique_digits.iter().filter(|d| d.len() == 2).next().unwrap();
            let four = unique_digits.iter().filter(|d| d.len() == 4).next().unwrap();
            let seven = unique_digits.iter().filter(|d| d.len() == 3).next().unwrap();
            let eight = unique_digits.iter().filter(|d| d.len() == 7).next().unwrap();

            // first get the unique ones (1, 7, 4, 8)
            populate_digit_segments(&mut digit_segments, 1, one);
            populate_digit_segments(&mut digit_segments, 7, seven);
            populate_digit_segments(&mut digit_segments, 4, four);
            populate_digit_segments(&mut digit_segments, 8, eight);

            // 'a' = seven - one
            populate_segment_wiring(&mut segment_wiring, 'a', get_first_diff(&digit_segments, 7, 1));

            let six_seg_digits = unique_digits
                .iter()
                .filter(|d| d.len() == 6)
                .collect::<Vec<_>>();

            // six - one leaves us with 5 segments (while nine or zero - 1 leaves us with 4)
            let six = six_seg_digits.iter().filter(|d| {
                let set = d.chars().collect::<HashSet<char>>();
                return set.difference(digit_segments.get(&1).unwrap()).count() == 5;
            }).next().unwrap();
            populate_digit_segments(&mut digit_segments, 6, six);
            // c = eight - six
            populate_segment_wiring(&mut segment_wiring, 'c', get_first_diff(&digit_segments, 8, 6));

            // f = one - (the mapping to c we just found)
            let mut one_segments = digit_segments.get(&1).unwrap().clone();
            one_segments.remove(segment_wiring.get(&'c').unwrap());
            populate_segment_wiring(&mut segment_wiring, 'f', *one_segments.iter().next().unwrap());

            // nine - four leaves us with 2 segments
            let nine = six_seg_digits.iter().filter(|d| {
                let set = d.chars().collect::<HashSet<char>>();
                return set.difference(digit_segments.get(&4).unwrap()).count() == 2;
            }).next().unwrap();
            populate_digit_segments(&mut digit_segments, 9, nine);
            // e = eight - nine
            populate_segment_wiring(&mut segment_wiring, 'e', get_first_diff(&digit_segments, 8, 9));

            // zero by process of elimination
            let zero = six_seg_digits.iter().filter(|d| {
                let set = d.chars().collect::<HashSet<char>>();
                return set != six.chars().collect::<HashSet<char>>() && set != nine.chars().collect::<HashSet<char>>();
            }).next().unwrap();
            populate_digit_segments(&mut digit_segments, 0, zero);
            // d = four - zero
            populate_segment_wiring(&mut segment_wiring, 'd', get_first_diff(&digit_segments, 4, 0));

            let nine_chars = digit_segments.get(&9).unwrap().clone();
            let eight_chars = digit_segments.get(&8).unwrap().clone();

            // five = nine - { 'c' mapping }
            for char in nine_chars.iter() {
                if char == segment_wiring.get(&'c').unwrap() { continue; }
                digit_segments.get_mut(&5).unwrap().insert(*char);
            }

            // union_for_g = seven U four U { 'e' mapping }
            let mut union_for_g = digit_segments.get(&7).unwrap()
                .union(&digit_segments.get(&4).unwrap())
                .collect::<HashSet<_>>();
            union_for_g.insert(segment_wiring.get(&'e').unwrap());
            let union_for_g = union_for_g.iter().map(|&x| *x).collect::<HashSet<char>>();

            // g = eight - union_for_g
            let g_segment_wiring = digit_segments.get(&8).unwrap()
                .difference(&union_for_g).next().unwrap();
            populate_segment_wiring(&mut segment_wiring, 'g', *g_segment_wiring);

            // b by process of elimination
            let b_segment_wiring = "abcdefg".chars().filter(|c| {
                return segment_wiring.iter().all(|sw| sw.1 != c);
            }).next().unwrap();
            populate_segment_wiring(&mut segment_wiring, 'b', b_segment_wiring);

            // three = nine - { 'b' mapping }
            for char in nine_chars.iter() {
                if char == segment_wiring.get(&'b').unwrap() { continue; }
                digit_segments.get_mut(&3).unwrap().insert(*char);
            }
            // two = eight - ({ 'b' mapping } U { 'f' mapping })
            for char in eight_chars {
                if &char == segment_wiring.get(&'b').unwrap() || &char == segment_wiring.get(&'f').unwrap() { continue; }
                digit_segments.get_mut(&2).unwrap().insert(char);
            }

            let to_add = output_digits.iter().enumerate().fold(0 as i32, |acc, (idx, output_digit)| {
                let multiplier = 1000 / ((10 as i32).pow(idx as u32)) as i32;
                let real_digit = *digit_segments.iter().filter(|x| {
                    if (*x).1.len() != output_digit.len() {
                        return false;
                    }

                    for c in output_digit.chars() {
                        if !(*x).1.contains(&c) {
                            return false;
                        }
                    }

                    true
                }).next().unwrap().0;

                acc + multiplier * real_digit
            });

            result += to_add;
        }

        println!("{}", result);
    }
}

fn populate_digit_segments(digit_segments: &mut HashMap<i32, HashSet<char>>, digit: i32, str: &str) {
    if let Some(d) = digit_segments.get_mut(&digit) {
        for char in str.chars() {
            d.insert(char);
        }
    }
}

fn populate_segment_wiring(segment_wiring: &mut HashMap<char, char>, segment: char, wiring: char) {
    segment_wiring.entry(segment).or_insert(wiring);
}

fn get_first_diff(digit_segments: &HashMap<i32, HashSet<char>>, key1: i32, key2: i32) -> char {
    *digit_segments.get(&key1).unwrap()
        .difference(digit_segments.get(&key2).unwrap())
        .next()
        .unwrap()
}

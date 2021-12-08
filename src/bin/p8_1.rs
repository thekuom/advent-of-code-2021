use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p8.txt") {
        let mut result = 0;
        let unique_segment_numbers: [usize; 4] = [2, 3, 4, 7]; // corresponding to 1, 7, 4, 8

        for line in lines {
            let line = line.unwrap();
            let outputs = line.split("|").last().unwrap().split(" ").collect::<Vec<_>>();

            result += outputs.iter().filter(|x| unique_segment_numbers.contains(&x.len())).count();
        }

        println!("{}", result);
    }
}
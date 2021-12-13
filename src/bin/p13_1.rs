use std::cmp::max;
use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p13.txt") {
        let mut paper = Vec::new();
        let mut fold_instructions = Vec::new();

        for line in lines {
            let line = line.unwrap();

            let splits = line.split(",").collect::<Vec<_>>();
            if splits.len() == 2 {
                let x = splits[0].parse::<usize>().unwrap();
                let y = splits[1].parse::<usize>().unwrap();

                if paper.len() < y + 1 {
                    paper.extend(vec![Vec::new(); y - paper.len() + 1]);
                }

                let row_len = paper[y].len();
                if row_len < x + 1 {
                    paper[y].extend(vec![false; x - row_len + 1]);
                }

                paper[y][x] = true;
            } else if line.contains("fold") {
                fold_instructions.push(line[("fold along ".len())..].to_string());
            }
        }

        let mut width = get_width(&paper);
        let mut height = paper.len();

        for row in paper.iter_mut() {
            if row.len() < width {
                row.extend(vec![false; width - row.len()]);
            }
        }

        let mut new_paper= paper;
        for fold in fold_instructions {
            let fold = fold.as_str().split("=").collect::<Vec<_>>();
            let axis = fold[0];
            let value = fold[1].parse::<usize>().unwrap();

            new_paper = if axis == "y" {
                (0..value)
                    .map(|row_idx| (0..width)
                        .map(|col_idx| new_paper[row_idx][col_idx] || new_paper[height - 1 - row_idx][col_idx])
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            } else {
                (0..height)
                    .map(|row_idx| (0..value)
                        .map(|col_idx| new_paper[row_idx][col_idx] || new_paper[row_idx][width - 1 - col_idx])
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            };

            width = get_width(&new_paper);
            height = new_paper.len();
        }

        print_paper(&new_paper);
    }
}

fn get_width(paper: &Vec<Vec<bool>>) -> usize {
    paper.iter().fold(0, |current_max, row|
        max(current_max, row.len())
    )
}

fn print_paper(paper: &Vec<Vec<bool>>) {
    for row in paper {
        let to_print = row.iter().map(|&col| if col { '#' } else { '.' }).collect::<String>();
        println!("{}", to_print);
    }
    println!();
}
use std::collections::HashSet;
use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p9.txt") {
        let mut matrix: Vec<Vec<i32>> = Vec::new();

        for line in lines {
            let line = line.unwrap();

            let mut row = Vec::new();

            for char in line.chars() {
                let num = char.to_string().parse::<i32>().unwrap();
                row.push(num);
            }

            matrix.push(row);
        }

        let low_points = get_low_points(&matrix);
        let mut basins = get_basins(&matrix, &low_points);
        basins.sort();
        basins.reverse();

        println!("{}", basins[0] * basins[1] * basins[2]);
    }
}

fn get_basins(matrix: &Vec<Vec<i32>>, low_points: &Vec<(usize, usize, i32)>) -> Vec<usize> {
    low_points.iter().map(|point| {
        let mut basin_points = HashSet::new();
        populate_points_in_basin(&matrix, &mut basin_points, (point.0, point.1));

        basin_points.len()
    }).collect()
}

fn populate_points_in_basin(
    matrix: &Vec<Vec<i32>>,
    covered_indices: &mut HashSet<(usize, usize)>,
    current_index: (usize, usize)) {
    covered_indices.insert(current_index);

    let adjacent_points = get_adjacent_points(&matrix, current_index);

    for point in adjacent_points.iter().filter_map(|&x| x) {
        if !covered_indices.contains(&(point.0, point.1)) && point.2 != 9 {
            populate_points_in_basin(matrix, covered_indices, (point.0, point.1));
        }
    }
}

fn get_low_points(matrix: &Vec<Vec<i32>>) -> Vec<(usize, usize, i32)> {
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    (0..num_rows).fold(Vec::new(), |mut row_acc, row_idx| {
        row_acc.extend((0..num_cols).fold(Vec::new(), |mut col_acc, col_idx| {
            let point = matrix[row_idx][col_idx];
            let adjacent_points = get_adjacent_points(&matrix, (row_idx, col_idx));

            if adjacent_points.iter().all(|p| match p {
                Some(x) => point < x.2,
                None => true
            }) {
                col_acc.push((row_idx, col_idx, point));
            }

            col_acc
        }));

        row_acc
    })
}

fn get_adjacent_points(matrix: &Vec<Vec<i32>>, row_col: (usize, usize)) -> [Option<(usize, usize, i32)>; 4] {
    let row_index = row_col.0;
    let col_index = row_col.1;

    [
        match row_index {
            0 => None,
            _ => Some((row_index - 1, col_index, matrix[row_index - 1][col_index])),
        },
        matrix[row_index].get(col_index + 1).map(|p| (row_index, col_index + 1, *p)),
        match matrix.get(row_index + 1) {
            Some(row) => Some((row_index + 1, col_index, row[col_index])),
            _ => None,
        },
        match col_index {
            0 => None,
            _ => Some((row_index, col_index - 1, matrix[row_index][col_index - 1])),
        }
    ]
}

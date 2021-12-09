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

        let result = get_low_points(&matrix).iter().fold(0, |acc, &item| {
            acc + item.2 + 1
        });

        println!("{}", result);
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
                Some(x) => point < *x,
                None => true
            }) {
                col_acc.push((row_idx, col_idx, point));
            }

            col_acc
        }));

        row_acc
    })
}

fn get_adjacent_points(matrix: &Vec<Vec<i32>>, row_col: (usize, usize)) -> [Option<i32>; 4] {
    let row_index = row_col.0;
    let col_index = row_col.1;

    [
        match row_index {
            0 => None,
            _ => Some(matrix[row_index - 1][col_index]),
        },
        matrix[row_index].get(col_index + 1).cloned(),
        match matrix.get(row_index + 1) {
            Some(row) => Some(row[col_index]),
            _ => None,
        },
        match col_index {
            0 => None,
            _ => Some(matrix[row_index][col_index - 1]),
        }
    ]
}
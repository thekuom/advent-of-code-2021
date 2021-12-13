use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p11.txt") {
        let mut dumbo_array: [[i32; 10]; 10] = Default::default();

        for (line_idx, line) in lines.enumerate() {
            let line = line.unwrap();

            for (char_idx, char) in line.chars().enumerate() {
                dumbo_array[line_idx][char_idx] = char.to_string().parse::<i32>().unwrap();
            }
        }

        let mut step_num = 1;
        loop {
            increase_energy(&mut dumbo_array);

            let mut flashes_in_step = 0;
            let mut flashing_octopi = get_flashing_octopi_indices(&dumbo_array);

            while flashing_octopi.len() > 0 {
                flashes_in_step += flashing_octopi.len();
                if flashes_in_step == 100 {
                    println!("{}", step_num);
                    return;
                }

                for idx in flashing_octopi {
                    dumbo_array[idx.0][idx.1] = 0;
                    increase_adjacent_octopi(&mut dumbo_array, idx);
                }

                flashing_octopi = get_flashing_octopi_indices(&dumbo_array);
            }

            step_num += 1;
        }
    }
}

fn get_flashing_octopi_indices(dumbo_array: &[[i32; 10]; 10]) -> Vec<(usize, usize)> {
    dumbo_array
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| row
            .iter()
            .enumerate()
            .filter(|(_col_idx, &col)| col > 9)
            .map(move |(col_idx, _col)| (row_idx, col_idx)))
        .collect::<Vec<_>>()
}

fn increase_energy(dumbo_array: &mut [[i32; 10]; 10]) {
    for i in 0..10 {
        for j in 0..10 {
            dumbo_array[i][j] += 1;
        }
    }
}

fn increase_adjacent_octopi(dumbo_array: &mut[[i32; 10]; 10], index: (usize, usize)) {
    let (x, y) = index;

    let adjacent_indices = [
        // N
        match y {
            0 => None,
            _ => Some((x, y - 1)),
        },
        // NE
        match (x, y) {
            (9, _) => None,
            (_, 0) => None,
            _ => Some((x + 1, y - 1)),
        },
        // E
        match x {
            9 => None,
            _ => Some((x + 1, y)),
        },
        // SE
        match (x, y) {
            (9, _) => None,
            (_, 9) => None,
            _ => Some((x + 1, y + 1)),
        },
        // S
        match y {
            9 => None,
            _ => Some((x, y + 1)),
        },
        // SW
        match (x, y) {
            (0, _) => None,
            (_, 9) => None,
            _ => Some((x - 1, y + 1)),
        },
        // W
        match x {
            0 => None,
            _ => Some((x - 1, y)),
        },
        // NW
        match (x, y) {
            (0, _) => None,
            (_, 0) => None,
            _ => Some((x - 1, y - 1)),
        },
    ];

    for (xi, yi) in adjacent_indices.iter().filter_map(|&x| x) {
        if dumbo_array[xi][yi] != 0 {
            dumbo_array[xi][yi] += 1;
        }
    }
}

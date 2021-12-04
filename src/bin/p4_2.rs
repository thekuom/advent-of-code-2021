use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p4.txt") {
        let mut bingo_input = String::new();
        let mut bingo_boards = Vec::new();

        for (idx, line) in lines.enumerate() {
            if idx == 0 {
                bingo_input = line.unwrap();
                continue;
            }

            if line.as_ref().unwrap().len() == 0 {
                bingo_boards.push(BingoBoard::new());
                continue;
            }

            let bingo_boards_len = bingo_boards.len();
            bingo_boards[bingo_boards_len - 1].add_row(line
                .unwrap()
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect());
        }

        let mut result = None;
        for input in bingo_input.split(",") {
            let input = input.parse::<usize>().unwrap();

            for board in bingo_boards.iter_mut() {
                if board.already_won { continue; }
                board.mark_number(input);

                if board.is_bingo() {
                    board.already_won = true;
                    result = Some(board.get_unmarked_squares_sum() * input);
                }
            }
        }

        println!("{}", result.unwrap());
    }
}

#[derive(Debug)]
struct BingoBoard {
    rows: Vec<Vec<BingoBoardSquare>>,
    cols: Vec<Vec<BingoBoardSquare>>,
    already_won: bool,
}

#[derive(Debug)]
struct BingoBoardSquare {
    value: usize,
    marked: bool,
}

impl BingoBoard {
    pub fn new() -> Self {
        BingoBoard {
            rows: Vec::new(),
            cols: vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            already_won: false,
        }
    }

    pub fn add_row(&mut self, row_data: Vec<usize>) {
        self.rows.push(row_data
            .iter()
            .map(|&x| BingoBoardSquare { value: x, marked: false })
            .collect::<Vec<BingoBoardSquare>>());

        for (idx, &val) in row_data.iter().enumerate() {
            self.cols[idx].push(BingoBoardSquare { value: val, marked: false });
        }
    }

    pub fn mark_number(&mut self, number: usize) {
        for row in self.rows.iter_mut() {
            for square in row.iter_mut() {
                if square.value == number {
                    square.marked = true;
                }
            }
        }

        for col in self.cols.iter_mut() {
            for square in col.iter_mut() {
                if square.value == number {
                    square.marked = true;
                }
            }
        }
    }

    pub fn is_bingo(&self) -> bool {
        self.rows.iter().any(|x| x.iter().all(|y| y.marked)) ||
            self.cols.iter().any(|x| x.iter().all(|y| y.marked))
    }

    pub fn get_unmarked_squares_sum(&self) -> usize {
        self.rows.iter()
            .flat_map(|x| x.iter().filter(|y| !y.marked))
            .fold(0, |acc, x| acc + x.value)
    }
}

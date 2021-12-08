pub fn print_answers() {
    println!("\n--- Day 4: Giant Squid ---");
    let mut bingo =
        BingoSubsystem::new(&std::fs::read_to_string("assets\\day_04_input.txt").unwrap());
    println!(
        "What will your final score be if you choose the board which will win first? {}",
        bingo.get_winning_score()
    );
    println!(
        "Once the last board wins, what would its final score be? {}",
        bingo.get_last_score()
    );
}

#[derive(Debug)]
struct BingoSubsystem {
    numbers: Vec<u16>,
    boards: Vec<BingoBoard>,
}

impl BingoSubsystem {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let numbers: Vec<u16> = lines[0]
            .split(",")
            .map(|n| n.parse::<u16>().unwrap())
            .collect();
        let board_count = (lines.len() - 1) / 6;
        let mut boards = Vec::with_capacity(board_count);
        for i in 0..board_count {
            let start_index = i * 6 + 2;
            boards.push(BingoBoard::new(&lines[start_index..start_index + 5]));
        }
        Self { numbers, boards }
    }

    fn get_winning_score(&mut self) -> u64 {
        for n in self.numbers.iter() {
            for board in self.boards.iter_mut() {
                board.mark_number(*n);
                if board.has_won() {
                    return board.get_score(*n);
                }
            }
        }

        panic!("no winning board")
    }

    fn get_last_score(&mut self) -> u64 {
        let mut i = 0;
        let mut remaining_boards = self.boards.clone();
        while remaining_boards.len() > 1 {
            for board in remaining_boards.iter_mut() {
                board.mark_number(self.numbers[i]);
            }
            remaining_boards.retain(|board| !board.has_won());
            i += 1;
        }
        while !remaining_boards[0].has_won() {
            remaining_boards[0].mark_number(self.numbers[i]);
            i += 1;
        }
        remaining_boards[0].get_score(self.numbers[i - 1])
    }
}

#[derive(Clone, Debug)]
struct BingoBoard {
    grid: Vec<(u16, bool)>,
}

impl BingoBoard {
    fn new(lines: &[&str]) -> Self {
        let grid = lines
            .iter()
            .map(|&line| {
                line.split_whitespace()
                    .map(|n| (n.parse::<u16>().unwrap(), false))
            })
            .flatten()
            .collect();
        Self { grid }
    }

    fn mark_number(&mut self, n: u16) {
        if let Some(tile) = self.grid.iter_mut().find(|tile| tile.0 == n) {
            tile.1 = true
        }
    }

    fn is_tile_marked(&self, column: usize, row: usize) -> bool {
        self.grid[row * 5 + column].1
    }

    fn check_column(&self, column: usize) -> bool {
        (0..5).all(|row| self.is_tile_marked(column, row))
    }

    fn check_columns(&self) -> bool {
        (0..5).any(|column| self.check_column(column))
    }

    fn check_row(&self, row: usize) -> bool {
        (0..5).all(|column| self.is_tile_marked(column, row))
    }

    fn check_rows(&self) -> bool {
        (0..5).any(|row| self.check_row(row))
    }

    fn has_won(&self) -> bool {
        self.check_columns() || self.check_rows()
    }

    fn get_score(&self, last_called: u16) -> u64 {
        self.grid
            .iter()
            .filter(|&&tile| !tile.1)
            .map(|&tile| tile.0 as u64)
            .sum::<u64>()
            * last_called as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_board_mark_number() {
        let mut board = BingoBoard::new(&INPUT.lines().skip(2).take(6).collect::<Vec<&str>>());
        board.mark_number(7);
        assert!(board.is_tile_marked(4, 2))
    }

    #[test]
    fn test_board_has_won() {
        let mut board = BingoBoard::new(&INPUT.lines().skip(14).take(6).collect::<Vec<&str>>());
        for n in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24] {
            board.mark_number(n);
        }
        assert!(board.has_won())
    }

    #[test]
    fn test_board_score() {
        let mut bingo = BingoSubsystem::new(&INPUT);
        assert_eq!(bingo.get_winning_score(), 4512)
    }

    #[test]
    fn test_last_board_score() {
        let mut bingo = BingoSubsystem::new(&INPUT);
        assert_eq!(bingo.get_last_score(), 1924)
    }
}

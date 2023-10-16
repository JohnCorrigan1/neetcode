use std::collections::{HashMap, HashSet};

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert_eq!(valid_sudoku(board), true);
}

pub fn valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut boxes: HashMap<usize, HashSet<char>> = HashMap::new();

    for i in 0..board.len() {
        rows.insert(i, HashSet::new());
        let row = rows.get_mut(&i).unwrap();
        for j in 0..board[i].len() {
            if board[i][j] == '.' {
                continue;
            }
            if let Some(_n) = row.get(&board[i][j]) {
                return false;
            } else {
                row.insert(board[i][j]);
            }
            if let Some(col) = cols.get_mut(&j) {
                if let Some(_n) = col.get(&board[i][j]) {
                    return false;
                } else {
                    col.insert(board[i][j]);
                }
            } else {
                let mut col = HashSet::new();
                col.insert(board[i][j]);
                cols.insert(j, col);
            }

            let box_index = (i / 3) * 3 + j / 3;
            if let Some(square) = boxes.get_mut(&box_index) {
                if let Some(_n) = square.get(&board[i][j]) {
                    return false;
                } else {
                    square.insert(board[i][j]);
                }
            } else {
                let mut square = HashSet::new();
                square.insert(board[i][j]);
                boxes.insert(box_index, square);
            }
        }
    }

    println!("valid: {}", true);

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn real() {
        let mut board: Vec<Vec<char>> = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        assert_eq!(valid_sudoku(board), true);
    }
}

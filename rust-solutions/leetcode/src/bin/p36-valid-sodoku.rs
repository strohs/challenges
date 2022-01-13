/// # Problem 36. Valid Sodoku
/// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated
/// according to the following rules:
/// 1. Each row must contain the digits 1-9 without repetition.
/// 2. Each column must contain the digits 1-9 without repetition.
/// 3. Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
///
/// Note:
/// - A Sudoku board (partially filled) could be valid but is not necessarily solvable.
/// - Only the filled cells need to be validated according to the mentioned rules.
///
/// ## Example 1
/// Input: board =
/// ```
/// [["5","3",".",".","7",".",".",".","."]
/// ,["6",".",".","1","9","5",".",".","."]
/// ,[".","9","8",".",".",".",".","6","."]
/// ,["8",".",".",".","6",".",".",".","3"]
/// ,["4",".",".","8",".","3",".",".","1"]
/// ,["7",".",".",".","2",".",".",".","6"]
/// ,[".","6",".",".",".",".","2","8","."]
/// ,[".",".",".","4","1","9",".",".","5"]
/// ,[".",".",".",".","8",".",".","7","9"]]
/// ```
/// Output: true

// the max columns in the board, the board is assumed to be square
const COL_SIZE: usize = 9;
// the character that indicates an empty cell on the board
const EMPTY_CELL: char = '.';

/// returns true if the specified row of the board, between start inclusive and end
/// exclusive, contains a duplicate element
fn row_has_duplicate(start: usize, end: usize, board: &[char]) -> bool {
    use std::collections::HashSet;
    let mut hash_set: HashSet<char> = HashSet::with_capacity(COL_SIZE);

    board[start..end]
        .iter()
        .any(|ch| *ch != EMPTY_CELL && !hash_set.insert(*ch))
}

/// returns true if the specified column of the board, that begins at `start` inclusive, and
/// ends at `end` exclusive, contains a duplicate element.
fn col_has_duplicate(start: usize, end: usize, board: &[char]) -> bool {
    use std::collections::HashSet;
    let mut hash_set: HashSet<char> = HashSet::with_capacity(COL_SIZE);

    board[start..end]
        .iter()
        .step_by(COL_SIZE)
        .any(|ch| *ch != EMPTY_CELL && !hash_set.insert(*ch))
}

/// returns true if a square sub-grid of board has a duplicate element
fn subgrid_has_duplicate(start: usize, sub_length: usize, board: &[char]) -> bool {
    use std::collections::HashSet;
    let mut hash_set: HashSet<char> = HashSet::with_capacity(COL_SIZE);
    let mut start = start;
    let end = start + (COL_SIZE * (sub_length - 1)) + sub_length;

    while start < end {
        let row_dupe = board[start..(start + sub_length)]
            .iter()
            .any(|ch| *ch != EMPTY_CELL && !hash_set.insert(*ch));
        if row_dupe {
            return true;
        } else {
            start += COL_SIZE;
        }
    }
    false
}

/// returns true if any rows of the board have duplicates
fn any_rows_have_duplicates(board: &[char]) -> bool {
    (0..board.len())
        .step_by(COL_SIZE)
        .into_iter()
        .any(|start| row_has_duplicate(start, start + COL_SIZE, board))
}

/// returns true if any of columns of the board have duplicates
fn any_cols_have_duplicates(board: &[char]) -> bool {
    (0..COL_SIZE).into_iter().any(|start| {
        let col_end = start + (COL_SIZE * (COL_SIZE - 1)) + 1;
        col_has_duplicate(start, col_end, board)
    })
}

fn any_sub_grids_have_duplicates(board: &[char]) -> bool {
    // subgrids start at  0, 3, 6  27, 30, 33,  54, 57, 60
    [0, 3, 6, 27, 30, 33, 54, 57, 60]
        .iter()
        .any(|&start| subgrid_has_duplicate(start, 3, board))
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // flatten the vector
    let board = board.into_iter().flatten().collect::<Vec<char>>();
    !(any_rows_have_duplicates(&board[..])
        || any_cols_have_duplicates(&board[..])
        || any_sub_grids_have_duplicates(&board[..]))
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{col_has_duplicate, is_valid_sudoku, row_has_duplicate, subgrid_has_duplicate};

    #[test]
    fn row_0_has_no_dupes() {
        let grid: Vec<Vec<char>> = vec![
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
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(!row_has_duplicate(0, 9, &fgrid));
    }

    #[test]
    fn row_7_has_dupes() {
        let grid: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '4', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(row_has_duplicate(63, 72, &fgrid));
    }

    #[test]
    fn co1_1_has_no_dupes() {
        let grid: Vec<Vec<char>> = vec![
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
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(!col_has_duplicate(1, 74, &fgrid));
    }

    #[test]
    fn co1_1_has_dupes() {
        let grid: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '3', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(col_has_duplicate(1, 74, &fgrid));
    }

    #[test]
    fn subgrid_has_no_dupes() {
        let grid: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '3', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(!subgrid_has_duplicate(0, 3, &fgrid));
    }

    #[test]
    fn subgrid_0_has_dupes() {
        let grid: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '5', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '3', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(subgrid_has_duplicate(0, 3, &fgrid));
    }

    #[test]
    fn subgrid_4_has_dupes() {
        let grid: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '2', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '6', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '3', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(subgrid_has_duplicate(30, 3, &fgrid));
    }

    #[test]
    fn subgrid_8_has_no_dupes() {
        let grid: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '2', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '3', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let fgrid: Vec<char> = grid.into_iter().flatten().collect();
        assert!(!subgrid_has_duplicate(60, 3, &fgrid));
    }

    #[test]
    fn board_is_valid() {
        let grid: Vec<Vec<char>> = vec![
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
        assert!(is_valid_sudoku(grid));
    }

    #[test]
    fn board_is_not_valid() {
        let grid: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '9', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(grid));
    }
}

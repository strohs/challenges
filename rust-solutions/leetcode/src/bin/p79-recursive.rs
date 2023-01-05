/// # LeetCode 79 - Word Search - Recursive Solution
/// given a 2D board of letters and a word, determine if the word exists on the board.
/// words can occur horizontally and vertically, but not diagonally.
///
/// ## Example
/// ```
/// board =
/// [
///   ['A','B','C','E'],
///   ['S','F','C','S'],
///   ['A','D','E','E']
/// ]
///
/// Given word = "ABCCED", return true.
/// Given word = "SEE", return true.
/// Given word = "ABCB", return false.
/// ```

/// wrapper for a 2D Vec
struct Matrix<T>(Vec<Vec<T>>);

impl<T> Matrix<T> {
    /// returns an iterator over `(row,col)` tuple, of the cells that are adjacent to the given row,col index
    fn adjacent_iter(&self, row: usize, col: usize) -> AdjacentIter<T> {
        AdjacentIter::new(&self, row, col)
    }
}

/// helper struct for iterating over the cells to the North, South, East, and West of the given row,col within a Matrix
struct AdjacentIter<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
    col: usize,
    // the current adjacent position we are about to iterate over next, 0 = North, 1 = East, 2 = South, 3 = West, 4 or more means we are done iterating
    pos: u8,
}

impl<'a, T> AdjacentIter<'a, T> {
    fn new(matrix: &'a Matrix<T>, row: usize, col: usize) -> Self {
        Self {
            matrix,
            row,
            col,
            pos: 0,
        }
    }

    fn get_north(&self) -> Option<(usize, usize)> {
        if self.row > 0 && self.matrix.0.get(self.row - 1).and_then(|row| row.get(self.col)).is_some() {
            Some((self.row - 1, self.col))
        } else {
            None
        }
    }

    fn get_east(&self) -> Option<(usize, usize)> {
        if self.matrix.0.get(self.row).and_then(|row| row.get(self.col+1)).is_some() {
            Some((self.row, self.col + 1))
        } else {
            None
        }
    }

    fn get_south(&self) -> Option<(usize, usize)> {
        if self.matrix.0.get(self.row + 1).and_then(|row| row.get(self.col)).is_some() {
            Some((self.row + 1, self.col))
        } else {
            None
        }
    }

    fn get_west(&self) -> Option<(usize, usize)> {
        if self.col > 0 && self.matrix.0.get(self.row).and_then(|row| row.get(self.col - 1)).is_some() {
            Some((self.row, self.col - 1))
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for AdjacentIter<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;
        while self.pos < 4 && next.is_none() {
            match self.pos {
                0 => next = self.get_north(),
                1 => next = self.get_east(),
                2 => next = self.get_south(),
                3 => next = self.get_west(),
                _ => next = None,
            }
            self.pos += 1;
        }
        next
    }
}



fn exist(mut board: Matrix<char>, word: String) -> bool {
    for i in 0..board.0.len() {
        for j in 0..board.0[0].len() {
            if exist_recursive(&mut board, &word, 0, i, j) {
                return true;
            }
        }
    }
    false
}

fn exist_recursive(board: &mut Matrix<char>, word: &str, idx: usize, i: usize, j: usize) -> bool {
    if word.chars().nth(idx).unwrap() != board.0[i][j] {
        println!();
        return false;
    }

    if idx == word.len() - 1 {
        return true;
    }
    let c = board.0[i][j];
    board.0[i][j] = '#';


    for (x, y) in board.adjacent_iter(i, j).collect::<Vec<(usize, usize)>>() {
        if idx + 1 < word.len() && board.0[x][y] == word.chars().nth(idx + 1).unwrap() {
            if exist_recursive(board, word, idx + 1, x, y) {
                return true;
            }
        }
    }
    board.0[i][j] = c;
    println!();
    false
}


fn main() {}

#[cfg(test)]
mod tests {
    use crate::exist;
    use super::AdjacentIter;
    use super::Matrix;

    #[test]
    fn test_adjacents_iter() {
        let b1 = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        let mut adjs = AdjacentIter::new(&b1, 1, 1);
        assert_eq!(adjs.next(), Some((0, 1)));
        assert_eq!(adjs.next(), Some((1, 2)));
        assert_eq!(adjs.next(), Some((2, 1)));
        assert_eq!(adjs.next(), Some((1, 0)));
        assert_eq!(adjs.next(), None);
    }

    #[test]
    fn test_adjacents_se_corner() {
        let mat = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        let mut adj = mat.adjacent_iter(2, 3);
        assert_eq!(adj.next(), Some((1, 3)));
        assert_eq!(adj.next(), Some((2, 2)));
        assert_eq!(adj.next(), None);
    }

    #[test]
    fn test_adjacents_nw_corner() {
        let mat = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        let mut adj = mat.adjacent_iter(0, 0);
        assert_eq!(adj.next(), Some((0, 1)));
        assert_eq!(adj.next(), Some((1, 0)));
        assert_eq!(adj.next(), None);
    }

    #[test]
    fn test_adjacents_center_north_corner() {
        let mat = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        let mut adj = mat.adjacent_iter(0, 2);
        assert_eq!(adj.next(), Some((0, 3)));
        assert_eq!(adj.next(), Some((1, 2)));
        assert_eq!(adj.next(), Some((0, 1)));
        assert_eq!(adj.next(), None);
    }

    #[test]
    fn test1() {
        let m1 = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        assert!(exist(m1, "E".to_string()));
    }

    #[test]
    fn test2() {
        let m1 = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        assert!(exist(m1, "ABCCED".to_string()));
    }

    #[test]
    fn test3() {
        let m1 = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        assert!(exist(m1, "SEE".to_string()));
    }

    #[test]
    fn test4() {
        let m1 = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        assert!(!exist(m1, "ABCB".to_string()));
    }

    #[test]
    fn test5() {
        let m2 = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'E', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        assert!(exist(m2, "ABCESEEEFS".to_string()));
    }

    #[test]
    fn test6() {
        let m2 = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'E', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        assert!(!exist(m2, "SEES".to_string()));
    }

    #[test]
    fn test7() {
        let m3 = Matrix(vec![
            vec!['a', 'a'],
            vec!['a', 'a']
        ]);
        assert!(!exist(m3, "aaaaa".to_string()));
    }

    #[test]
    fn test8() {
        let m4 = Matrix(vec![
            vec!['a', 'a', 'b', 'a', 'a', 'b'],
            vec!['a', 'a', 'b', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a', 'b', 'a'],
            vec!['b', 'a', 'b', 'b', 'a', 'b'],
            vec!['a', 'b', 'b', 'a', 'b', 'a'],
            vec!['b', 'a', 'a', 'a', 'a', 'b'],
        ]);
        assert!(exist(m4, "bbbaabbbbab".to_string()));
    }
}
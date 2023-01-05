use std::iter::from_fn;

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
    // fn adjacent_iter(&self, row: usize, col: usize) -> AdjacentIter<T> {
    //     AdjacentIter::new(&self, row, col)
    // }

    /// returns an iterator over `(row,col)` indices that are adjacent to the given `row,col` index
    fn adjacents(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        let mut adjs = Vec::with_capacity(4);
        if let Some(adj) = self.get_west(row, col) {
            adjs.push(adj);
        }
        if let Some(adj) = self.get_south(row, col) {
            adjs.push(adj);
        }
        if let Some(adj) = self.get_east(row, col) {
            adjs.push(adj);
        }
        if let Some(adj) = self.get_north(row, col) {
            adjs.push(adj);
        }
        from_fn(move || adjs.pop())
    }

    fn get_north(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row > 0 && self.0.get(row - 1).and_then(|row| row.get(col)).is_some() {
            Some((row - 1, col))
        } else {
            None
        }
    }

    fn get_east(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if self.0.get(row).and_then(|row| row.get(col + 1)).is_some() {
            Some((row, col + 1))
        } else {
            None
        }
    }

    fn get_south(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if self.0.get(row + 1).and_then(|row| row.get(col)).is_some() {
            Some((row + 1, col))
        } else {
            None
        }
    }

    fn get_west(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if col > 0 && self.0.get(row).and_then(|row| row.get(col - 1)).is_some() {
            Some((row, col - 1))
        } else {
            None
        }
    }
}

// /// helper struct for iterating over the cells to the North, South, East, and West of the given row,col within a Matrix
// struct AdjacentIter<'a, T> {
//     matrix: &'a Matrix<T>,
//     row: usize,
//     col: usize,
//     // the current adjacent position we are about to iterate over next, 0 = North, 1 = East, 2 = South, 3 = West, 4 or more means we are done iterating
//     pos: u8,
// }
//
// impl<'a, T> AdjacentIter<'a, T> {
//     fn new(matrix: &'a Matrix<T>, row: usize, col: usize) -> Self {
//         Self {
//             matrix,
//             row,
//             col,
//             pos: 0,
//         }
//     }
//
//
// }
//
// impl<'a, T> Iterator for AdjacentIter<'a, T> {
//     type Item = (usize, usize);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let mut next = None;
//         while self.pos < 4 && next.is_none() {
//             match self.pos {
//                 0 => next = self.get_north(),
//                 1 => next = self.get_east(),
//                 2 => next = self.get_south(),
//                 3 => next = self.get_west(),
//                 _ => next = None,
//             }
//             self.pos += 1;
//         }
//         next
//     }
// }

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

    for (x, y) in board.adjacents(i, j) {
        if idx + 1 < word.len()
            && board.0[x][y] == word.chars().nth(idx + 1).unwrap()
            && exist_recursive(board, word, idx + 1, x, y)
        {
            return true;
        }
    }
    board.0[i][j] = c;
    println!();
    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::exist;

    #[test]
    fn test_adjacents_se_corner() {
        let mat = Matrix(vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]);
        let mut adj = mat.adjacents(2, 3);
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
        let mut adj = mat.adjacents(0, 0);
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
        let mut adj = mat.adjacents(0, 2);
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
        let m3 = Matrix(vec![vec!['a', 'a'], vec!['a', 'a']]);
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

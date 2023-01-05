/// # LeetCode 79 - Word Search
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

/// This is an iterative solution rather than recursive
fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    /// get (up to) four 'cells' adjacent to the cell at index: r,c
    /// # returns
    /// a 3-tuple containing the adjacent cells' row/col indices and the character at that index
    fn adj(board: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<(usize, usize, char)> {
        let mut adjs: Vec<(usize, usize, char)> = Vec::with_capacity(4);

        // get north and south cell
        if r > 0 {
            if let Some(row) = board.get(r - 1) {
                if let Some(ch) = row.get(c) {
                    adjs.push((r - 1, c, *ch));
                }
            }
            if let Some(row) = board.get(r + 1) {
                if let Some(ch) = row.get(c) {
                    adjs.push((r + 1, c, *ch));
                }
            }
        } else {
            // only get south
            if let Some(row) = board.get(r + 1) {
                if let Some(ch) = row.get(c) {
                    adjs.push((r + 1, c, *ch));
                }
            }
        }

        // get west and east
        if c > 0 {
            // get west
            if let Some(row) = board.get(r) {
                if let Some(ch) = row.get(c - 1) {
                    adjs.push((r, c - 1, *ch))
                }
            }
            // get east
            if let Some(row) = board.get(r) {
                if let Some(ch) = row.get(c + 1) {
                    adjs.push((r, c + 1, *ch))
                }
            }
        } else {
            // only get east
            if let Some(row) = board.get(r) {
                if let Some(ch) = row.get(c + 1) {
                    adjs.push((r, c + 1, *ch))
                }
            }
        }

        adjs
    }

    /// find and return all row,col indices of `ch` within the *board*
    fn find_all_indices(board: &Vec<Vec<char>>, ch: char) -> Vec<(usize, usize)> {
        let cdim = board[0].len();
        board
            .iter()
            .flatten()
            .enumerate()
            .filter(|&(_idx, c)| *c == ch)
            .map(|(i, _c)| (i / cdim, i % cdim)) // convert 1d idx to 2d
            .collect::<Vec<(usize, usize)>>()
    }

    /// builds a Vec of chars from the characters that have been visited on the board
    fn charify(visited: &[(usize, usize)], board: &Vec<Vec<char>>) -> Vec<char> {
        visited.iter().map(|&(r, c)| board[r][c]).collect()
    }

    /// depth first search
    /// starting at row,col, try to find a path on *board* that matches **ALL** chars in *word*
    fn dfs(board: &Vec<Vec<char>>, row: usize, col: usize, word: &[char]) -> bool {
        // list of board indices to be visited
        let mut to_visit: Vec<(usize, usize)> = vec![];
        // list of indices that have been visited (in the current branch)
        let mut visited: Vec<(usize, usize)> = vec![];
        // list of indices of the current visited path, where we could branch
        let mut branches: Vec<usize> = vec![];
        // the current character of word that is being examined
        let mut i: usize = 0;

        // the initial Idx to visit
        to_visit.push((row, col));

        while i < word.len() && !to_visit.is_empty() {
            let curr_idx = to_visit.pop().unwrap();
            let (cr, cc) = curr_idx;

            if !visited.contains(&curr_idx) {
                // add current index to list of visited
                visited.push(curr_idx);

                if &charify(&visited, board) == word {
                    return true;
                }

                // fetch adjacent adjacent indices that match the next character of word, that have not been visited
                if word.get(i + 1).is_some() {
                    let mut adjs = adj(board, cr, cc)
                        .iter()
                        .filter(|(r, c, ch)| *ch == word[i + 1] && !visited.contains(&(*r, *c)))
                        .map(|&(r, c, _ch)| (r, c))
                        .collect::<Vec<(usize, usize)>>();

                    // no more matching characters found, see if we can rollback to a branching point
                    if adjs.is_empty() {
                        if let Some(branch_idx) = branches.pop() {
                            visited.truncate(branch_idx + 1);
                            i = branch_idx + 1;
                        }
                    } else {
                        // found 1 or more adjacent matching letters, increment i
                        i += 1;
                    }

                    // going down a new branch, mark last index of visited as a branch index
                    if adjs.len() > 1 {
                        branches.push(visited.len() - 1);
                    }
                    // add any adjacent indexes into the to_visit list
                    to_visit.append(&mut adjs);
                }
            }
        }
        false
    }

    let letters = word.chars().collect::<Vec<char>>();
    // get all row/col indices that match the first letter of the word
    let start_pos = find_all_indices(&board, letters[0]);

    start_pos
        .into_iter()
        .any(|(row, col)| dfs(&board, row, col, &letters))
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::exist;

    #[test]
    fn test1() {
        let b1 = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(exist(b1.clone(), "E".to_string()));
    }

    #[test]
    fn test2() {
        let b1 = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(exist(b1.clone(), "ABCCED".to_string()));
    }

    #[test]
    fn test3() {
        let b1 = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(exist(b1.clone(), "SEE".to_string()));
    }

    #[test]
    fn test4() {
        let b1 = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(!exist(b1.clone(), "ABCB".to_string()));
    }

    #[test]
    fn test5() {
        let b2 = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'E', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(exist(b2.clone(), "ABCESEEEFS".to_string()));
    }

    #[test]
    fn test6() {
        let b2 = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'E', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(!exist(b2.clone(), "SEES".to_string()));
    }

    #[test]
    fn test7() {
        let b3 = vec![vec!['a', 'a'], vec!['a', 'a']];
        assert!(!exist(b3.clone(), "aaaaa".to_string()));
    }

    #[test]
    fn test8() {
        let b3 = vec![
            vec!['a', 'a', 'b', 'a', 'a', 'b'],
            vec!['a', 'a', 'b', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a', 'b', 'a'],
            vec!['b', 'a', 'b', 'b', 'a', 'b'],
            vec!['a', 'b', 'b', 'a', 'b', 'a'],
            vec!['b', 'a', 'a', 'a', 'a', 'b'],
        ];
        assert!(!exist(b3.clone(), "bbbaabbbbab".to_string()));
    }
}

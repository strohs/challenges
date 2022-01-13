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

/// get (up to) four 'cells' adjacent to the cell at index: r,c
/// # returns
/// a 3-tuple containing the adjacent cells' row/col indices and the character at that index
fn adj(board: &[Vec<char>], r: usize, c: usize) -> Vec<(usize, usize, char)> {
    // safely get a value from a 2d Vec
    let get2d = |ri: usize, ci: usize| match board.get(ri) {
        Some(row) if row.get(ci).is_some() => Some((ri, ci, *row.get(ci).unwrap())),
        _ => None,
    };

    // push (r, c+1) and (r+1, c)
    let mut adjs = vec![
        get2d(r, c + 1),
        get2d(r + 1, c)
    ];

    // get r,c-1
    if c > 0 {
        adjs.push(get2d(r, c - 1));
    }
    // get r-1, c
    if r > 0 {
        adjs.push(get2d(r - 1, c));
    }

    adjs.into_iter()
        .filter(|&e| e.is_some())
        .map(|e| e.unwrap())
        .collect::<Vec<(usize, usize, char)>>()
}

/// find and return all index pairs of *ch* on the *board*
fn find_all_indices(board: &[Vec<char>], ch: char) -> Vec<(usize, usize)> {
    let cdim = board[0].len();
    board
        .iter()
        .flatten()
        .enumerate()
        .filter(|&(_idx, c)| *c == ch)
        .map(|(i, _c)| (i / cdim, i % cdim)) // convert 1d idx to 2d
        .collect::<Vec<(usize, usize)>>()
}

/// depth first search
/// starting at row,col, try to find a path on *board* that matches **ALL** chars in *word*
fn dfs(board: &[Vec<char>], row: usize, col: usize, word: &[char]) -> bool {
    let mut to_visit: Vec<(usize, usize)> = vec![];
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut i: usize = 0;

    to_visit.push((row, col));

    while i < word.len() && !to_visit.is_empty() {
        let (cr, cc) = to_visit.pop().unwrap();
        if !visited.contains(&(cr, cc)) {
            visited.push((cr, cc));
            if board[cr][cc] == word[i] {
                i += 1;
            }
            // fetch matching adjacent chars from our current board position
            if word.get(i).is_some() {
                let mut adjs = adj(board, cr, cc)
                    .iter()
                    .filter(|(_r, _c, ch)| *ch == word[i])
                    .map(|&(r, c, _ch)| (r, c))
                    .collect::<Vec<(usize, usize)>>();
                if adjs.is_empty() {
                    i -= 1;
                }
                to_visit.append(&mut adjs)
            }
        }
    }
    i == word.len()
}

fn exists(board: &[Vec<char>], word: String) -> bool {
    let letters = word.chars().collect::<Vec<char>>();
    // get all row/col indices that match the first letter of the word
    let start_pos = find_all_indices(board, letters[0]);

    start_pos
        .into_iter()
        .any(|(row, col)| dfs(board, row, col, &letters))
}

fn main() {
    let b1 = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    dbg!(exists(&b1, "E".to_string()));
    dbg!(exists(&b1, "ABCCED".to_string())); // true
    dbg!(exists(&b1, "SEE".to_string())); // true
    dbg!(exists(&b1, "ABCB".to_string())); // false
}

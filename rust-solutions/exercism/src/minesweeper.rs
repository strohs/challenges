/// # Minesweeper
/// ## Instructions
/// Add the mine counts to a completed Minesweeper board.
/// Minesweeper is a popular game where the user has to find the mines using numeric hints
/// that indicate how many mines are directly adjacent (horizontally, vertically, diagonally) to a square.
///
/// In this exercise you have to create some code that counts the number of mines adjacent
/// to a given empty square and replaces that square with the count.
/// The board is a rectangle composed of blank space (' ') characters. A mine is represented
/// by an asterisk ('*') character.
///
/// If a given space has no adjacent mines at all, leave that square blank.

const BLANK: char = ' ';
const MINE: char = '*';

/// returns the row,col indices of squares adjacent to the given row,col index
/// Panics if the given r,c values are >= MAX_ISIZE || <= MIN_ISIZE
fn neighbor_indices(r: usize, c: usize, row_len: usize, col_len: usize) -> Vec<(usize, usize)> {
    const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    let row_len = row_len as isize;
    let col_len = col_len as isize;
    let r = r as isize;
    let c = c as isize;

    return NEIGHBOR_OFFSETS
        .iter()
        .map(|&(ro, co)| (ro + r, co + c))
        .filter(|&(ro, co)| ro >= 0 && co >= 0 && ro < row_len && co < col_len)
        .map(|(ro, co)| (ro as usize, co as usize))
        .collect();
}

/// returns a count of the number of mines adjacent to the square at r, c
fn adj_mine_count(r: usize, c: usize, minefield: &[&str]) -> usize {
    // compute the indices of neighboring squares to r,c
    let nbrs = neighbor_indices(r, c, minefield.len(), minefield[0].len());
    // compute the count of neighbors containing mines
    nbrs.into_iter().fold(0_usize, |count, (nr, nc)| {
        let square = minefield[nr].chars().nth(nc).unwrap();
        if square == MINE {
            count + 1
        } else {
            count
        }
    })
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut annotated_minefield: Vec<String> = vec![];
    if minefield.is_empty() {
        return annotated_minefield;
    }
    let rows = minefield.len();
    let cols = minefield[0].len();

    for r in 0..rows {
        let mut row_str = String::new();
        for c in 0..cols {
            let square = minefield[r].chars().nth(c).unwrap();
            if square == BLANK {
                let adj_count = adj_mine_count(r, c, &minefield);
                if adj_count > 0 {
                    row_str.push_str(&adj_count.to_string());
                } else {
                    row_str.push(square);
                }
            } else {
                row_str.push(square);
            }
        }
        annotated_minefield.push(row_str);
    }

    annotated_minefield
}

#[cfg(test)]
mod tests {
    use super::neighbor_indices;
    use crate::minesweeper::annotate;

    #[test]
    fn test_neighbor_indices() {
        let sl = &[[" ", " ", " "], [" ", "*", " "], [" ", " ", " "]];
        let nbrs = neighbor_indices(0, 0, sl.len(), sl[0].len());
        assert_eq!(nbrs.len(), 3);
        assert!(nbrs.contains(&(1, 1)));
        assert!(nbrs.contains(&(0, 1)));
        assert!(nbrs.contains(&(1, 0)));
    }

    #[test]
    fn dbg_annotated_minefield() {
        let sl = &["   ", " * ", "  *"];
        let ann = annotate(sl);
        dbg!(&ann);
    }

    #[test]
    fn only_mines() {
        let sl = &["***", "***", "***"];
        let ann = annotate(sl);
        dbg!(&ann);
    }

    #[test]
    fn no_rows() {
        let sl = &[];
        let ann = annotate(sl);
        dbg!(&ann);
    }
}

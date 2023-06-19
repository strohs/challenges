

/// # Leetcode 130 - Surrounded Regions
/// https://leetcode.com/problems/surrounded-regions/
///

fn neighbors(board: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut ns: Vec<(usize, usize)> = vec![];
    if board.get(row).and_then(|r| r.get(col + 1)).is_some() {
        ns.push((row, col + 1));
    }
    if board.get(row).and_then(|r| r.get(col - 1)).is_some() {
        ns.push((row, col - 1));
    }
    if board.get(row + 1).and_then(|r| r.get(col)).is_some() {
        ns.push((row + 1, col));
    }
    if board.get(row - 1).and_then(|r| r.get(col)).is_some() {
        ns.push((row - 1, col));
    }
    ns
}

fn dfs(board: &Vec<Vec<char>>, srow: usize, scol: usize) -> Vec<(usize, usize)> {
    use std::collections::HashSet;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: Vec<(usize, usize)> = vec![(srow, scol)];

    while !to_visit.is_empty() {
        let (r, c) = to_visit.pop().unwrap();
        if !visited.contains(&(r, c)) {
            let o_neighbors = neighbors(&board, r, c)
                .into_iter()
                .filter(|(nr, nc)| board[*nr][*nc] == 'O')
                .collect::<Vec<(usize, usize)>>();
            for (nr, nc) in o_neighbors {
                if nr != 0 && nr != board.len() - 1 && nc != 0 && nc != board[0].len() - 1 {
                    if !visited.contains(&(nr, nc)) {
                        to_visit.push((nr, nc));
                    }
                } else {
                    return vec![];
                }
            }
        }
        visited.insert((r, c));
    }
    visited.into_iter().collect()
}

pub fn solve(board: &mut Vec<Vec<char>>) {
    for row in 1..board.len() - 1 {
        for col in 1..board[0].len() - 1 {
            if board[row][col] == 'O' {
                let surrounded = dfs(&board, row, col);
                for (sr, sc) in surrounded {
                    board[sr][sc] = 'X';
                }
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn example1() {
        let mut board = vec![
            vec!['X','X','X','X'],
            vec!['X','O','O','X'],
            vec!['X','X','O','X'],
            vec!['X','O','X','X']
        ];
        let output = vec![
            vec!['X','X','X','X'],
            vec!['X','X','X','X'],
            vec!['X','X','X','X'],
            vec!['X','O','X','X']
        ];
        solve(&mut board);
        assert_eq!(board, output);
    }

    #[test]
    fn example2() {
        let mut board = vec![
            vec!['X'],
        ];
        let output = vec![
            vec!['X'],
        ];
        solve(&mut board);
        assert_eq!(board, output);
    }
}
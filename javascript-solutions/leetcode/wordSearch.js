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

// get adjacent indices around board[r][c]
function adjacents(board, r, c) {
    const rdim = board.length;
    const cdim = board[0].length;
    const adjs = [];

    // add r,c+1
    if (c < cdim - 1) adjs.push({ r: r, c: c+1 });
    // add r+1, c
    if (r < rdim - 1) adjs.push({ r: r+1, c: c });
    // add r, c-1
    if (c > 0) adjs.push({ r: r, c: c-1 });
    // r-1, c
    if (r > 0) adjs.push({ r: r-1, c: c });

    return adjs;
}

// return all indices of char on the board
function find_all_indices_of_char(board, ch) {
    const matches = [];
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[0].length; j++) {
            if (board[i][j] === ch) matches.push({ r: i, c: j });
        }
    }
    return matches;
}

// depth first search
function dfs(board, row, col, word) {
    let toVisit = [];
    const visited = [];
    let i = 0;

    toVisit.push({r: row, c: col});

    while (i < word.length && toVisit.length > 0) {
        const idx = toVisit.pop();
        if (!visited.includes(idx)) {
            visited.push(idx);
            if (board[idx.r][idx.c] === word[i]) {
                i += 1;
            }
            // get indices of the next letter in the word
            if (i <= word.length - 1) {
                const adjs = adjacents(board, idx.r, idx.c).filter(({r,c}) => board[r][c] === word[i]);
                if (adjs.length === 0) {
                    i -= 1;
                }
                toVisit = toVisit.concat(adjs);
            }
        }
    }
    return i === word.length;
}

function exist(board, word) {
    return find_all_indices_of_char(board, word[0]).some(({r,c}) => dfs(board,r,c,word));
}

const b1 = [
    ['A','B','C','E'],
    ['S','F','C','S'],
    ['A','D','E','E']
];

console.log( exist(b1, "SEE") );    //true
console.log( exist(b1, "ABCCED") ); //true
console.log( exist(b1, "ABFE") );   //false
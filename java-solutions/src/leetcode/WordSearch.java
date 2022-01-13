package leetcode;/// # LeetCode 79 - Word Search
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

import java.util.*;

public class WordSearch {

    // holds a row,col index
    private class Idx {
        public Idx(int r, int c) {
            this.r = r;
            this.c = c;
        }
        int r; //row
        int c; //col

        @Override
        public String toString() {
            return "(" + r + ", " + c + ')';
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Idx idx = (Idx) o;
            return r == idx.r &&
                    c == idx.c;
        }

        @Override
        public int hashCode() {
            return Objects.hash(r, c);
        }
    }

    // return adjacent indices around r,c that match the character in 'match'
    private List<Idx> adjacents(char[][] board, int r, int c, char match) {
        int rdim = board.length;
        int cdim = board[0].length;
        List<Idx> adjs = new ArrayList<>(4);

        if (c < cdim - 1 && board[r][c+1] == match)
            adjs.add( new Idx(r, c+1) );
        if (r < rdim - 1 && board[r+1][c] == match)
            adjs.add( new Idx(r+1, c));
        if (c > 0 && board[r][c-1] == match)
            adjs.add( new Idx(r, c-1) );
        if (r > 0 && board[r-1][c] == match)
            adjs.add( new Idx(r-1, c));
        return adjs;
    }

    // return all indices in board, that match ch
    private List<Idx> find_all_indices(char[][] board, char ch) {
        List<Idx> matches = new ArrayList<>();
        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                if (board[i][j] == ch)
                    matches.add( new Idx(i,j));
            }
        }
        return matches;
    }

    // depth first search
    private boolean dfs(char[][] board, int row, int col, String word) {
        Stack<Idx> toVisit = new Stack<>();
        List<Idx> visited = new ArrayList<>();
        char [] letters = word.toCharArray();
        int i = 0;

        toVisit.add(new Idx(row, col));

        while (i < letters.length && !toVisit.isEmpty()) {
            Idx idx = toVisit.pop();
            if (!visited.contains(idx)) {
                visited.add(idx);
                if (board[idx.r][idx.c] == letters[i]) {
                    i += 1;
                }
                // fetch adjacent cells that match the next letter
                if (i <= letters.length - 1) {
                    List<Idx> adjs = adjacents(board, idx.r, idx.c, letters[i]);
                    if (adjs.isEmpty()) {
                        i -= 1;
                    }
                    toVisit.addAll(adjs);
                }
            }
        }
        return i == word.length();
    }

    public boolean exist(char[][] board, String word) {
        List<Idx> firstChars = find_all_indices(board, word.charAt(0));
        return firstChars.stream().anyMatch(idx -> dfs(board, idx.r, idx.c, word));
    }

    public static void main(String[] args) {
        WordSearch ws = new WordSearch();
        char [][] b1 = {
                {'A','B','C','E'},
                {'S','F','C','S'},
                {'A','D','E','E'}
        };

        System.out.println(ws.exist(b1, "SEE"));   // true
        System.out.println(ws.exist(b1, "ABCCED"));// true
        System.out.println(ws.exist(b1, "ABCB"));  // false
    }
}

package leetcode.Problem79;

import java.util.*;
import java.util.stream.Collectors;

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



public class Solution {

    /**
     * Helper class that stores a row,col index
     */
    private static class Idx {
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

    /**
     * returns a List of Idx objects that are to the N,S,E,W of the given r,c index in the given board
     */
    List<Idx> adjacents(char[][] board, int r, int c) {
        int row_length = board.length;
        int col_length = board[0].length;

        // holds the Indexes of N,S,E,W adjacent cells to the given r,c index
        List<Idx> index_deltas = new ArrayList<>(4);
        index_deltas.add(new Idx(r - 1, c));
        index_deltas.add(new Idx(r + 1, c));
        index_deltas.add(new Idx(r, c - 1));
        index_deltas.add(new Idx(r, c + 1));

        // filter out any indices that are out of bounds of the board
        return index_deltas.stream()
                .filter(idx -> idx.r > -1 && idx.r < row_length && idx.c > -1 && idx.c < col_length)
                .collect(Collectors.toList());
    }


    // return all indices in board, that match ch
    public List<Idx> find_all_indices(char[][] board, char ch) {
        List<Idx> matches = new ArrayList<>();
        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                if (board[i][j] == ch)
                    matches.add(new Idx(i, j));
            }
        }
        return matches;
    }

//    void dbg(char[][] board, ArrayList<Idx> visited) {
//        for (Idx idx : visited) {
//            System.out.printf("%s:(%d, %d)  ", board[idx.r][idx.c], idx.r, idx.c);
//        }
//        System.out.println();
//    }

    // depth first search
    public boolean dfs(char[][] board, int row, int col, String word) {
        ArrayList<Idx> toVisit = new ArrayList<>();
        ArrayList<Idx> visited = new ArrayList<>();
        ArrayList<Integer> branches = new ArrayList<>();
        char [] letters = word.toCharArray();
        int i = 0;

        toVisit.add(new Idx(row, col));

        while (i < letters.length && !toVisit.isEmpty()) {
            Idx idx = toVisit.remove(toVisit.size() - 1);

            if (!visited.contains(idx)) {
                // add current idx to list of visited indices
                visited.add(idx);

                if (i + 1 == letters.length) {
                    return true;
                }

                // fetch adjacent cells that match the next letter of letters, AND have not been visited
                if (i + 1 <= letters.length) {
                    final char next_letter = letters[i+1];
                    List<Idx> adjs = adjacents(board, idx.r, idx.c)
                            .stream()
                            .filter(ix -> board[ix.r][ix.c] == next_letter && !visited.contains(ix))
                            .collect(Collectors.toList());

                    if (adjs.isEmpty()) {
                        if (!branches.isEmpty()) {
                            // rollback the current visited path to the last branch point
                            int branch_idx = branches.remove(branches.size() - 1);
                            int remove_count = visited.size() - branch_idx - 1;
                            while (remove_count > 0) {
                                visited.remove(visited.size() - 1);
                                remove_count -= 1;
                            }
                            // rollback the current word character
                            i = branch_idx + 1;
                        }
                    } else {
                        i += 1;
                    }

                    // new branch(es) found, nark the last index of visited as a branching index
                    if (adjs.size() > 1) {
                        for (int j = 0; j < adjs.size() - 1; j++) {
                            branches.add(visited.size() - 1);
                        }
                    }
                    // add all adjacent indices to to_visit
                    toVisit.addAll(adjs);
                }
            }
        }
        return false;
    }


    public boolean exist(char[][] board, String word) {
        List<Idx> firstChars = find_all_indices(board, word.charAt(0));
        return firstChars
                .stream()
                .anyMatch(idx -> dfs(board, idx.r, idx.c, word));
    }

    public static void main(String[] args) {
        Solution ws = new Solution();
        char [][] b1 = {
                {'A','B','C','E'},
                {'S','F','C','S'},
                {'A','D','E','E'}
        };

        System.out.println(ws.exist(b1, "SEE"));   // true
        System.out.println(ws.exist(b1, "ABCCED"));// true
        System.out.println(ws.exist(b1, "ABCB"));  // false


        char[][] b2 = {
            {'A','B','C','E'},
            {'S','F','E','S'},
            {'A','D','E','E'}
        };
        System.out.println(ws.exist(b2, "ABCESEEEFS"));   // true

        char[][] b3 = {
                {'a','a','b','a','a','b'},
                {'a','a','b','b','b','a'},
                {'a','a','a','a','b','a'},
                {'b','a','b','b','a','b'},
                {'a','b','b','a','b','a'},
                {'b','a','a','a','a','b'}
        };
        System.out.println(ws.exist(b3, "bbbaabbbbbab"));   // false

        char[][] b4 = {
                {'A','B','C','E'},
                {'S','F','E','S'},
                {'A','D','E','E'}
        };
        System.out.println(ws.exist(b4, "SEES"));   // false

        char[][] b5 = {
                {'A','A','A','A','A','A'},
                {'A','A','A','A','A','A'},
                {'A','A','A','A','A','A'},
                {'A','A','A','A','A','A'},
                {'A','A','A','A','A','A'},
                {'A','A','A','A','A','A'},
        };
        System.out.println(ws.exist(b5, "AAAAAAAAAAAABAA"));   // false
    }
}

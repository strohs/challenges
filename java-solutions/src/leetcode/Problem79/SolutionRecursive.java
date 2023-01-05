package leetcode.Problem79;

public class SolutionRecursive {

    // 1.DFS backtracking
    // Runtime: 2 ms, faster than 99.76% of Java online submissions for Word Search.
    // Memory Usage: 41.9 MB, less than 68.68% of Java online submissions for Word Search.
    // Time: O(N * (3 ^ L)); Space: O(L)
    // let N be the number of cells in the board.
    // Let L be the length of the word
    public boolean exist(char[][] board, String word) {

        int flag = 0;
        for (char[] chars : board) {
            for (int j = 0; j < board[0].length; j++) {
                if (chars[j] == word.charAt(0)) flag++;
                else if (chars[j] == word.charAt(word.length() - 1)) flag--;
            }
        }

        // flag is a heuristic that indicates that there are fewer ending chars than beginning chars on the board.
        // In this case, we reverse the word and find a path from the last char of word to the beginning char.
        // This will reduce the number of positions on the board that we have to examine.
        if (flag > 0)
            word = new StringBuilder(word).reverse().toString();

        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                if (helper_backtrack(board, word, 0, i, j)) return true;
            }
        }
        return false;
    }

    int[][] pos = new int[][]{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
    // Time: O(L); Space: O(L)
    // Let L be the length of word
    private boolean helper_backtrack(char[][] board, String word, int idx, int i, int j) {

        if (word.charAt(idx) != board[i][j])
            return false;
        if (idx == word.length() - 1)
            return true;

        char c = board[i][j];
        board[i][j] = '#'; // mark the board position as visited

        for (int[] p : pos) {
            int x = i + p[0], y = j + p[1];
            if (x < 0 || x >= board.length || y < 0 || y >= board[0].length)
                continue;

            if (idx + 1 < word.length() && board[x][y] == word.charAt(idx + 1))
                if (helper_backtrack(board, word, idx + 1, x, y))
                    return true;
        }

        board[i][j] = c; // restore the board position
        return false;
    }

    public static void main(String[] args) {
        SolutionRecursive sol = new SolutionRecursive();
        char[][] m4 = new char[][] {
                {'a', 'a', 'b', 'a', 'a', 'b'},
                {'a', 'a', 'b', 'b', 'b', 'a'},
                {'a', 'a', 'a', 'a', 'b', 'a'},
                {'b', 'a', 'b', 'b', 'a', 'b'},
                {'a', 'b', 'b', 'a', 'b', 'a'},
                {'b', 'a', 'a', 'a', 'a', 'b'},
        };
        System.out.println(sol.exist(m4, "bbbaabbbbab"));
    }
}

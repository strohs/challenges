package leetcode;

/**
 * Given a `m x n` grid filled with non-negative numbers, find a path from top left
 * to bottom right, which minimizes the sum of all numbers along its path.
 *
 * Note: You can only move either down or right at any point in time.
 */
public class P64MinPathSum {

    public int minPathSum(int[][] grid) {
        int row_len = grid.length;
        int col_len = grid[0].length;

        // allocate a temporary matrix to hold computed sums
        int [][] sums = new int [row_len][col_len];

        // prefill leftmost column, column 0, with sum of the current cell value, plus value of cell above it
        int cur_sum = 0;
        for (int r=0; r < row_len; r++) {
            cur_sum += grid[r][0];
            sums[r][0] = cur_sum;
        }

        // prefill top row with current cells value + value of cell to its left
        cur_sum = 0;
        for (int c=0; c < col_len; c++) {
            cur_sum += grid[0][c];
            sums[0][c] = cur_sum;
        }

        // now iterate the remaining "inner" cells to find the minimum sum of that cell
        //  for each cell, its value will be the minimum of: (cur_cell + top_cell, cur_cell + left_cell)
        for (int r = 1; r < row_len; r++) {
            for (int c = 1; c < col_len; c++) {
                int left_cell_sum = grid[r][c] + sums[r][c-1];
                int top_cell_sum = grid[r][c] + sums[r-1][c];
                sums[r][c] = Integer.min(left_cell_sum, top_cell_sum);
            }
        }

        return sums[row_len-1][col_len-1];
    }

    public static void main(String[] args) {
        P64MinPathSum sol = new P64MinPathSum();
        int [][] grid = {
                {1, 3, 1},
                {1, 5, 1},
                {4, 2, 1}
        };
        int answer = sol.minPathSum(grid);
        System.out.println(answer);
        assert answer == 7;
    }
}

package leetcode;

import java.util.ArrayList;
import java.util.List;

/**
 * Leetcode Problem 118 - Pascal's Triangle
 * https://leetcode.com/problems/pascals-triangle/
 */
public class P118PascalsTriangle {

    public List<List<Integer>> generate(int numRows) {
        List<List<Integer>> rows = new ArrayList<>();

        for (int n = 0; n < numRows; n++) {
            if (n == 0) {
                rows.add(List.of(1));
            } else {
                List<Integer> row = new ArrayList<>();
                row.add(1);
                for (int i = 1; i <= n; i++) {
                    // potential overflow here, so might want to convert to long
                    int cur = (row.get(i - 1) * (n - i + 1)) / i;
                    row.add(cur);
                }
                rows.add(row);
            }
        }
        return rows;
    }
}

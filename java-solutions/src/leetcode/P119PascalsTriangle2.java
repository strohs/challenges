package leetcode;

import java.util.ArrayList;
import java.util.List;

/**
 * Leetcode Problem 119 - Pascals Triangle
 * https://leetcode.com/problems/pascals-triangle-ii/
 *
 */
public class P119PascalsTriangle2 {
    public List<Integer> getRow(int rowIndex) {
        List<Integer> row = new ArrayList<>();

        if (rowIndex == 0) {
            row.add(1);
        } else {
            row.add(1);
            for (int i = 1; i <= rowIndex; i++) {
                // need to handle possible overflow by converting to long first
                long numerator = (long) row.get(i - 1) * (long) (rowIndex - i + 1);
                int cur = (int) (numerator / (long) i);
                row.add(cur);
            }
        }
        return row;
    }

    public static void main(String[] args) {
        P119PascalsTriangle2 sol = new P119PascalsTriangle2();
        var res = sol.getRow(30);
        System.out.println(res);
    }
}

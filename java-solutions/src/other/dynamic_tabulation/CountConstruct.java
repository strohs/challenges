package other.dynamic_tabulation;

import java.util.HashMap;

public class CountConstruct {

    /**
     * returns the number of ways that target could be built from strings.
     * This class will use a dynamic programming table approach
     * <p>
     * Complexity:
     * m = target.length
     * n = strings.length
     * <p>
     * Brute Force:
     *  time: O(m^2 * n) -
     *  mem: O(m) - the size of the dp array
     */
    int countConstruct(String target, String[] strings) {
        int[] dp = new int[target.length() + 1];
        dp[0] = 1; // one way to create an empty string

        for (int i = 0; i < target.length(); i++) {
            String curTarget = target.substring(i);
            for (String s: strings) {
                if (curTarget.startsWith(s) && i + s.length() <= dp.length) {
                    dp[i + s.length()] += dp[i];
                }
            }
        }
        return dp[target.length()];
    }

    public static void main(String[] args) {
        CountConstruct sol = new CountConstruct();
        String target = "abcdef";
        String [] strings = new String[] { "ab", "abc", "cd", "def", "abcd" };
        System.out.println(sol.countConstruct(target, strings)); // 1

        target = "aabbcc";
        strings = new String[] { "aa", "a", "bb", "cc" };
        System.out.println(sol.countConstruct(target, strings)); // 2

        target = "";
        strings = new String[] { "cat", "dog", "kevin" };
        System.out.println(sol.countConstruct(target, strings)); // 1

        target = "eee";
        strings = new String[] { "e", "ee", "eee" };
        System.out.println(sol.countConstruct(target, strings)); // 4

        target = "eeef";
        strings = new String[] { "e", "ee", "eee" };
        System.out.println(sol.countConstruct(target, strings)); // 0
    }
}

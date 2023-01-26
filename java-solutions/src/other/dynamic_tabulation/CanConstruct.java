package other.dynamic_tabulation;

import java.util.Arrays;

public class CanConstruct {

    /**
     * returns true if there is at least one way to construct 'target' using a concatenation of
     * the strings in 'strings'.
     * This solution using a tabular approach rather than recursion
     * <p>
     * Complexity:
     * m = target.length
     * n = strings.length
     * <p>
     * Brute Force:
     *  time: O(m*n*m) = O(m^2 n) - the extra "* m" is due to the startsWith call which could take at worst m time
     *  mem: O(m + m) = O(m) - m for the dp array and + m for the curTarget sub string
     */
    boolean canConstruct(String target, String[] strings) {
        boolean[] dp = new boolean[target.length() + 1];
        Arrays.fill(dp, false);
        dp[0] = true;

        for (int i = 0; i < target.length(); i++) {
            if (dp[i]) {
                String curTarget = target.substring(i);
                for (String s: strings) {
                    if (curTarget.startsWith(s) && i + s.length() <= target.length()) {
                        dp[i + s.length()] = true;
                    }
                }
            }
        }
        return dp[target.length()];
    }


    public static void main(String[] args) {
        CanConstruct sol = new CanConstruct();
        String target = "abcdef";
        String [] strings = new String[] { "ab", "abc", "cd", "def", "abcd" };
        System.out.println(sol.canConstruct(target, strings)); // true

        target = "skateboard";
        strings = new String[] { "bo", "rd", "ate", "t", "ska", "sk", "boar" };
        System.out.println(sol.canConstruct(target, strings)); // false

        target = "";
        strings = new String[] { "cat", "dog", "kevin" };
        System.out.println(sol.canConstruct(target, strings)); // true

        target = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef";
        strings = new String[] { "e", "ee", "eee", "eeee", "eeeee", "eeeeee", "eeeeeee" };
        System.out.println(sol.canConstruct(target, strings)); // false
    }
}

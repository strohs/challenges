package other.dynamic_memoization;

import java.util.HashMap;

public class CountConstruct {

    /**
     * returns the number of ways that target could be built from strings
     * <p>
     * Complexity:
     * m = target.length
     * n = strings.length
     * <p>
     * Brute Force:
     *  time: O(n^m * m) - * m because we are taking a substring (at worst of length m)
     *  mem: O(m * m) = O(m^2) - at worst we could allocate m stack frames plus take a substring of length m
     * Memoized:
     *  time: O(n * m^2)
     *  mem: O(m * m) = O(m^2)
     */
    int countConstruct(String target, String[] strings) {
        HashMap<String, Integer> memo = new HashMap<>();
        return helper(target, strings, memo);
    }

    int helper(String target, String[] strings, HashMap<String, Integer> memo) {
        if (memo.containsKey(target)) {
            return memo.get(target);
        }
        if (target.isEmpty()) {
            return 1;
        }

        int count = 0;
        for (String s: strings) {
            if (target.startsWith(s)) {
                String subTarget = target.substring(s.length());
                count += helper(subTarget, strings, memo);
                //memo.put(subTarget, count);
            }
        }

        memo.put(target, count);
        return count;
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
    }
}

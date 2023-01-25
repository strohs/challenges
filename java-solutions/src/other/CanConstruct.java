package other;

import java.util.HashMap;

public class CanConstruct {

    /**
     * returns true if there is at least one way to construct target using a concatenation of
     * the strings in strings
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
    boolean canConstruct(String target, String[] strings) {
        HashMap<String, Boolean> memo = new HashMap<>();
        return helper(target, strings, memo);
    }

    boolean helper(String target, String[] strings, HashMap<String, Boolean> memo) {
        if (memo.containsKey(target)) {
            return memo.get(target);
        }
        if (target.isEmpty()) {
            return true;
        }
        for (String s: strings) {
            if (target.startsWith(s)) {
                String subTarget = target.substring(s.length());
                if (helper(subTarget, strings, memo)) {
                    memo.put(subTarget, true);
                    return true;
                }
            }
        }

        // notice that the false case is last, this will only trigger after we have tried all branches and none of them
        // returned true
        memo.put(target, false);
        return false;
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

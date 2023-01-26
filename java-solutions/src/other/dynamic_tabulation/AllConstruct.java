package other.dynamic_tabulation;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class AllConstruct {

    /**
     * returns all elements of strings that can construct the target.
     * This approach uses dynamic programming tables to compute the answer
     *
     * <p>
     * Complexity:
     * m = target.length
     * n = strings.length
     * <p>
     * Brute Force:
     *  time: O(m^2 * n)
     *  mem: O(m*m*m)
     */

    List<List<String>> allConstruct(String target, String[] strings) {
        // init the dp list to a series of empty 2d lists
        List<List<List<String>>> dp = new ArrayList<>(target.length() + 1);
        for (int i = 0; i <= target.length(); i++) {
            List<List<String>> l2d = new ArrayList<>(0);
            dp.add(l2d);
        }
        List<List<String>> first = dp.get(0);
        first.add(new ArrayList<>());

        for (int i = 0; i <= target.length(); i++) {
            String curTarget = target.substring(i);
            for (String s : strings) {
                if (curTarget.startsWith(s) && i + s.length() <= dp.size()) {
                    // append s to every list of strings in dp[i] and then copy each of those to dp[i+s.length]
                    for (List<String> curCellList : dp.get(i)) {
                        dp.get(i + s.length()).add(concatString(curCellList, s));
                    }
                }
            }
        }
        return dp.get(target.length());
    }

    // concat a String onto a list and return a new list with the concatenation
    static List<String> concatString(List<String> l1, String s) {
        List<String> l2 = new ArrayList<>(l1);
        l2.add(s);
        return l2;
    }

    public static void main(String[] args) {
        AllConstruct sol = new AllConstruct();
        String target = "abcdef";
        String [] strings = new String[] { "ab", "abc", "cd", "def", "abcd" };
        List<List<String>> ans = sol.allConstruct(target, strings); // [abc, def]
        for (List<String> ls : ans) {
            System.out.println(ls);
        }

        target = "purple";
        strings = new String[] { "purp", "p", "ur", "le", "purpl" };
        System.out.println(sol.allConstruct(target, strings)); // [p, ur, p, le], [purp, le]

        target = "aabbcc";
        strings = new String[] { "aa", "a", "bb", "cc" };
        System.out.println(sol.allConstruct(target, strings)); // [aa, bb, cc], [a, a, bb, cc]

        target = "";
        strings = new String[] { "cat", "dog", "kevin" };
        System.out.println(sol.allConstruct(target, strings)); // [[]]

        target = "eee";
        strings = new String[] { "e", "ee", "eee" };
        System.out.println(sol.allConstruct(target, strings)); // [e,e,e] [e,ee] [ee,e] [eee]

        target = "eeeeeeeeeeeeef";
        strings = new String[] { "e", "ee", "eee", "eeee", "eeeee" };
        System.out.println(sol.allConstruct(target, strings)); // []
    }
}

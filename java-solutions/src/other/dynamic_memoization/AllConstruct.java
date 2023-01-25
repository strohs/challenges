package other.dynamic_memoization;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class AllConstruct {

    /**
     * returns all elements of strings that can construct the target.
     *
     * <p>
     * Complexity:
     * m = target.length
     * n = strings.length
     * <p>
     * Brute Force:
     *  time: O(n^m) -
     *  mem: O(m) - just taking into account the call stack space
     * Memoized:
     *  time:
     *  mem:
     */

    List<List<String>> allConstruct(String target, String[] strings) {
        HashMap<String, List<List<String>>> memo = new HashMap<>();

        return helper(target, strings, memo);
    }

    List<List<String>> helper(String target, String[] strings, Map<String, List<List<String>>> memo) {
        if (memo.containsKey(target)) {
            return memo.get(target);
        }

        if (target.isEmpty()) {
            List<List<String>> temp = new ArrayList<>();
            temp.add(new ArrayList<>(0));
            return temp;
        }

        List<List<String>> answers = new ArrayList<>();

        for (String s: strings) {
            if (target.startsWith(s)) {
                String suffix = target.substring(s.length()); // O(m)
                List<List<String>> suffixWays = helper(suffix, strings, memo);
                // prepend the current, s, to each suffixWay list
                List<List<String>> targetWays = suffixWays
                        .stream()
                        .map(way -> prependString(way, s))
                        .toList();
                // O(m)
                answers.addAll(targetWays);
                memo.put(target, answers);
            }
        }
        //memo.put(target, count);
        return answers;
    }

    // concat a String onto a list and return a new list with the concatenation
    static List<String> concatString(List<String> l1, String s) {
        List<String> l2 = new ArrayList<>(l1);
        l2.add(s);
        return l2;
    }

    static List<String> prependString(List<String> l1, String s) {
        List<String> l2 = new ArrayList<>();
        l2.add(s);
        l2.addAll(l1);
        return l2;
    }

    static List<List<String>> concat2dLists(List<List<String>> l1, List<List<String>> l2) {
        return Stream.concat(l1.stream(), l2.stream()).collect(Collectors.toList());
    }

    public static void main(String[] args) {
        AllConstruct sol = new AllConstruct();
        String target = "abcdef";
        String [] strings = new String[] { "ab", "abc", "cd", "def", "abcd" };
        List<List<String>> ans = sol.allConstruct(target, strings); // [abc, def]
        for (List<String> ls : ans) {
            System.out.println(ls);
        }

        target = "aabbcc";
        strings = new String[] { "aa", "a", "bb", "cc" };
        System.out.println(sol.allConstruct(target, strings)); // [aa, bb, cc], [a, a, bb, cc]

        target = "";
        strings = new String[] { "cat", "dog", "kevin" };
        System.out.println(sol.allConstruct(target, strings)); // []

        target = "eee";
        strings = new String[] { "e", "ee", "eee" };
        System.out.println(sol.allConstruct(target, strings)); // [e,e,e] [e,ee] [ee,e] [eee]

        target = "eeeeeeeeeeeeef";
        strings = new String[] { "e", "ee", "eee", "eeee", "eeeee" };
        System.out.println(sol.allConstruct(target, strings)); // []
    }
}

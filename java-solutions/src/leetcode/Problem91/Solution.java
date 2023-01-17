package leetcode.Problem91;

public class Solution {

    public int numDecodings(String s) {
        int [] ways = new int[s.length() + 1];
        ways[0] = 1;
        ways[1] = s.startsWith("0") ? 0 : 1;

        for (int i = 2; i <= s.length(); i++) {
            String first = s.substring(i-1, i);
            String second = s.substring(i-2, i);
            if (!first.startsWith("0")) {
                ways[i] += ways[i - 1];
            }
            if (second.startsWith("1")) {
                ways[i] += ways[i - 2];
            }
            if (second.startsWith("2") && second.charAt(1) <= '6') {
                ways[i] += ways[i - 2];
            }
        }
        return ways[s.length()];
    }

    public static void main(String[] args) {
        Solution sol = new Solution();
        System.out.println(sol.numDecodings("27")); // 1
        System.out.println(sol.numDecodings("226")); // 3
        System.out.println(sol.numDecodings("111111111111111111111111111111111111111111111")); // 1836311903
    }
}

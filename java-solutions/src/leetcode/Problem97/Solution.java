package leetcode.Problem97;

/**
 * Problem 97 - Interleaving Strings
 * <a href="https://leetcode.com/problems/interleaving-string/">Interleaving Strings</a>
 */
public class Solution {


    // This is a recursive solution that will take time O(2^m+n) where m = s1.length and n = s3.length
    // and O(m+n) space.
    // This solution works but will exceed leetcode's timelimit
    // TODO another approach using a 2d boolean memoization matrix: memo[i][j], have to recurse with i,j indices pointing to s1 and s2 respectively
    public boolean isInterleave(String s1, String s2, String s3) {
        return isInterleaveRec(s1, s2, s3);
    }

    boolean isInterleaveRec(String s1, String s2, String s3) {
        // base cases
        if (s1.isEmpty() && s2.isEmpty()) {
            return true;
        }
        if (s1.isEmpty()) {
            // check s2 only
            if (s2.charAt(0) == s3.charAt(0)) {
                return isInterleaveRec("", s2.substring(1), s3.substring(1));
            } else {
                return false;
            }
        }

        if (s2.isEmpty()) {
            // check s1 only
            if (s1.charAt(0) == s3.charAt(0)) {
                return isInterleaveRec(s1.substring(1), "", s3.substring(1));
            } else {
                return false;
            }
        }

        if (s1.charAt(0) == s3.charAt(0) && s2.charAt(0) == s3.charAt(0)) {
            // both s1 and s2 have a matching char
            return (isInterleaveRec(s1.substring(1), s2, s3.substring(1)) ||
                    isInterleaveRec(s1, s2.substring(1), s3.substring(1)));
        } else if (s1.charAt(0) == s3.charAt(0)) {
            // only s1 char matched
            return isInterleaveRec(s1.substring(1), s2, s3.substring(1));
        } else if (s2.charAt(0) == s3.charAt(0)) {
            // only s2 char matched
            return isInterleaveRec(s1, s2.substring(1), s3.substring(1));
        } else {
            // no chars from s1 or s2 matched
            return false;
        }

    }

    public static void main(String[] args) {
        Solution sol = new Solution();
        String s1 = "aabcc";
        String s2 = "dbbca";
        String s3 = "aadbbcbcac";
        System.out.println(sol.isInterleave(s1, s2, s3)); // true

        s1 = "aabcc";
        s2 = "dbbca";
        s3 = "aadbcbbcac";
        System.out.println(sol.isInterleave(s1, s2, s3)); // true

        s1 = "aabcc";
        s2 = "dbbca";
        s3 = "aadbbbaccc";
        System.out.println(sol.isInterleave(s1, s2 ,s3)); // false

        s1 = "";
        s2 = "dbbca";
        s3 = "dbbca";
        System.out.println(sol.isInterleave(s1, s2 ,s3)); // true

        s1 = "aabcc";
        s2 = "";
        s3 = "aabcc";
        System.out.println(sol.isInterleave(s1, s2 ,s3)); // true

        s1 = "";
        s2 = "";
        s3 = "";
        System.out.println(sol.isInterleave(s1, s2 ,s3)); // true
    }
}

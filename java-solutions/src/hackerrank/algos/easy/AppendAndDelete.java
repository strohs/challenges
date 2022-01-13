/// You have a string of lowercase English alphabetic letters. You can perform two types of operations on the string:
///
/// 1. Append a lowercase English alphabetic letter to the end of the string.
/// 2, Delete the last character in the string. Performing this operation on an empty string results in an empty string.
///
/// Given an integer, `k`, and two strings, `s` and `t`, determine whether or not you can
/// convert `s` to `t` by performing exactly `k` of the above operations on `s`. If it's possible,
/// print Yes. Otherwise, print No.

package hackerrank.algos.easy;


public class AppendAndDelete {


    // returns number of chars that matched in each string, returns the count once first non-matching char is found
    static int matchingCharCount(String s, String t) {
        int minLength = Math.min(s.length(), t.length());
        for (int i = 0; i < minLength; i++) {
            if (s.charAt(i) != t.charAt(i)) {
                return i;
            }
        }
        return minLength;
    }

    static boolean isEven(int n) {
        return n % 2 == 0;
    }

    static String appendAndDelete(String s, String t, int k) {

        if (s.length() + t.length() <= k) {
            // in this case, k is large enough so that we could delete all of s and then add all of t's chars to it
            return "Yes";
        } else {
            // need to first get a count of any matching prefix characters
            int count = matchingCharCount(s, t);

            if ( isEven( k - (s.length() - count + t.length() - count)) ) {
                return "Yes";
            } else {
                return "No";
            }
        }
    }

    public static void main(String[] args) {
        
        System.out.println(appendAndDelete("hackerhappy", "hackerrank", 9)); // yes

        System.out.println(appendAndDelete("aba", "aba", 6)); // yes

        System.out.println(appendAndDelete("ashley", "ash", 3)); // yes

        System.out.println(appendAndDelete("ash", "ashley", 3)); // yes

        System.out.println(appendAndDelete("xyz", "abcdefg", 7)); // no

        System.out.println(appendAndDelete("aaaaaaaaaa", "aaaaa", 7)); // yes

        System.out.println(appendAndDelete("aaaaaaaaaa", "aaaaa", 6)); // no

        System.out.println(appendAndDelete("aaa", "aaaaa", 2)); // yes

    }
}

package freecodecamp;

import java.util.HashMap;

/**
 * Determine of two strings: s1 and s2 are valid anagrams.
 * Two strings are valid anagrams if they contain the same characters along
 * with the same character frequencies.
 * For example, "danger" and "garden" are valid anagrams
 */
public class ValidAnagram {


    // generate a hashmap of characters in the strings to a count of the number
    // of times they occur
    public static HashMap<Character, Integer> frequencies(String s) {
        var freqs = new HashMap<Character, Integer>();

        for (int i = 0; i < s.length(); i++) {
            freqs.merge(s.charAt(i), 1, Integer::sum);
        }

        return freqs;
    }

    // 1. both strings must have same length
    // 2. compare maps for equality, using Java's Map.equals()
    public static boolean validAnagram(String s1, String s2) {
        // assuming strings will be non-null
        if (s1.length() == s2.length()) {
            var s1Freqs = ValidAnagram.frequencies(s1);
            var s2Freqs = ValidAnagram.frequencies(s2);
            return s1Freqs.equals(s2Freqs);
        } else {
            return false;
        }
    }

    public static void main(String[] args) {
        var s1 = "danger";
        var s2 = "garden";
        var s3 = "ranger";

        boolean valid = ValidAnagram.validAnagram(s1,s2);
        System.out.println(valid);
    }
}

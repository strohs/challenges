package leetcode.Problem443;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

/**
 * Leetcode 443 - String Compression
 * <a href="https://leetcode.com/problems/string-compression/description/">String Compression</a>
 */
public class StringCompression {

    public int compress(char[] chars) {

        int i = 0;
        int j = 0;
        while (j < chars.length) {
            char cur = chars[j];
            int count = 0;
            while (j < chars.length && cur == chars[j]) {
                count++;
                j++;
            }
            if (count == 1) {
                chars[i] = cur;
                i++;
            } else {
                // write out count data
                chars[i] = cur;
                i++;
                char[] countChars = String.valueOf(count).toCharArray();
                for (int k = 0; k < countChars.length; k++) {
                    chars[i + k] = countChars[k];
                }
                i += countChars.length;
            }
        }
        return i;
    }


    public static void main(String[] args) {
        StringCompression sol = new StringCompression();
        char[] chars = new char[] {'a', 'a', 'b', 'b', 'c', 'c', 'c'};
        int res = sol.compress(chars);
        System.out.println(res);
        System.out.println(chars);

        System.out.println(sol.compress(new char[] {'a'}));

        chars = new char[] {'a', 'a', 'a', 'b', 'b', 'a', 'a'};
        int res2 = sol.compress(chars);
        System.out.println(res2);
        System.out.println(chars);

        chars = new char[] {'a', 'a', 'a', 'a', 'a', 'b'};
        int res3 = sol.compress(chars);
        System.out.println(res3);
        System.out.println(chars);
    }
}

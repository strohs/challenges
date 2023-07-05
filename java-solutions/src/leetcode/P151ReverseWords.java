package leetcode;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

public class P151ReverseWords {

    public String reverseWords(String s) {
        List<String> toks = Arrays.stream(s.split(" "))
                .filter(t -> !t.isEmpty())
                .collect(Collectors.toList());
        Collections.reverse(toks);
        return String.join(" ", toks);
    }

    public static void main(String[] args) {
        P151ReverseWords sol = new P151ReverseWords();
        String s = new String("   hello   world   ");

        String res = sol.reverseWords(s);
        System.out.println(":" + res + ":");
    }
}

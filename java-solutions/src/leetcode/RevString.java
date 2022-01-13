package leetcode;

import java.time.temporal.ValueRange;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class RevString {

    public static void main(String[] args) {
        String s = new String("abcdefg");
        
        // reverse a string via for loop
        for (int i = s.length() - 1; i > -1; i--) {
            System.out.println(s.charAt(i));
        }

    }
}

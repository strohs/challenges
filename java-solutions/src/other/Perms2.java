package other;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

/**
 * Another recursive algorithm to generate the permutations of the characters in s String
 */
public class Perms2 {

    // gens = collects the final generated permutations
    // curPerm = the current permutation being generated
    // s = hold remaining characters that need to be permuted
    public static void permutations(ArrayList<String> gens, String curPerm, String s) {
        if (!s.isEmpty()) {
            for (int i = 0; i < s.length(); i++) {
                String nextPerm = curPerm + s.charAt(i);
                String remaining = s.substring(0, i) + s.substring(i+1);
                permutations(gens, nextPerm, remaining);
            }
        } else {
            gens.add(curPerm);
        }
    }

    public static void main(String[] args) {
        String t = "xyz";

        
        ArrayList <Character> chars = t.chars().mapToObj( i -> (char) i ).collect(Collectors.toCollection(ArrayList::new));

        List<Character> charList = List.of('a', 'b', 'c');
        String temp = charList.stream()
                .map(String::valueOf)
                .collect(Collectors.joining());
        System.out.println(temp);



        String s = "ABC";
        ArrayList<String> finals = new ArrayList<>();
        Perms2.permutations(finals, "", s);

        for (String perm: finals) {
            System.out.println(perm);
        }
    }
}

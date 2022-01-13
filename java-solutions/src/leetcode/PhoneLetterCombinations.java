package leetcode;/// # Letter Combinations of a phone number
/// Given a string containing digits from `2-9` inclusive, return all possible letter combinations
/// that the number could represent.
/// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that
/// 1 does not map to any letters.
/// ```
/// 2 -> a,b
/// 3 -> d,e,f
/// 4 -> g,h,i
/// 5 -> j,k,l
/// 6 -> m,n,o
/// 7 -> p,q,r,s
/// 8 -> t,u,v
/// 9 -> w,x,y,z
/// ```
/// ## Example
/// input: "23"
/// output: `["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]`
///
/// FYI permutation - you care about the order of the elements
///     n! / (n - k)!
/// combinations - you don't care about the order of the elements
///    combination formula, you have n objects and want to choose k    = n! / k!(n - k!)

import java.util.*;

public class PhoneLetterCombinations {

    private Map<Character, List<Character>> digitMap;

    private static Map<Character, List<String>> buildDigitMap() {
        Map<Character, List<String>> map = new HashMap<Character, List<String>>(8);
        map.put('2', Arrays.asList("a","b","c"));
        map.put('3', Arrays.asList("d","e","f"));
        map.put('4', Arrays.asList("g","h","i"));
        map.put('5', Arrays.asList("j","k","l"));
        map.put('6', Arrays.asList("m","n","o"));
        map.put('7', Arrays.asList("p","q","r","s"));
        map.put('8', Arrays.asList("t","u","v"));
        map.put('9', Arrays.asList("w","x","y","z"));
        return map;
    }

    // combine the Strings in s1 and s2 into a new list of combined strings
    private static List<String> cartesianProduct (List<String> s1, List<String> s2) {
        List<String> combis = new ArrayList<>();
        if (s1.isEmpty() && !s2.isEmpty()) {
            combis.addAll(s2);
        } else if (!s1.isEmpty() && s2.isEmpty()) {
            combis.addAll(s1);
        } else {
            for (String s : s1) {
                for (String t : s2) {
                    combis.add(s + t);
                }
            }
        }
        return combis;
    }

    public static void main (String[] args) {
        final Map<Character, List<String>> digitMap = buildDigitMap();
        String input1 = "223";
        List<String> str_digits = Arrays.asList(input1.split(""));

        Optional<List<String>> res = str_digits
                .stream()
                .map(digit -> digitMap.get(digit.charAt(0)))
                .reduce(PhoneLetterCombinations::cartesianProduct);
        

        System.out.println(res.get());

    }
}

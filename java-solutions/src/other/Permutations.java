package other;

/**
 * print the permutations of the characters in a String
 */
public class Permutations {

    void perms(String s, String ans) {
        if (s.isEmpty()) {
            System.out.println(ans);
            return;
        }

        for (int i = 0; i < s.length(); i++) {
            System.out.printf("  i:%d     s:%s   ans:%s\n", i, s, ans);
            // ith char of s
            char c = s.charAt(i);

            // rest of string, excluding the ith char
            String ros = s.substring(0, i) + s.substring(i+1);

            perms(ros, ans + c);
        }
    }

    public static void main(String[] args) {
        String str = "ABC";
        Permutations perm = new Permutations();
        perm.perms(str, "");
    }
}

package algorithms.easy;

public class StaircaseJava {

    public static void main(String[] args) {
        staircase(20);
    }

    public static void staircase(int n) {
        for (int i = n - 1; i >= 0 ; i--) {
            String repStr = repeat("#", n - i);
            System.out.println( padStart(repStr, n, ' ') );
        }
    }

    private static String repeat(String s, Integer n) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) {
            sb.append(s);
        }
        return sb.toString();
    }

    private static String padStart(String s, Integer n, Character c) {
        StringBuilder sb = new StringBuilder();
        int charLength = n - s.length();
        String preString = repeat(c.toString(), charLength);
        sb.append(preString);
        sb.append(s);
        return sb.toString();
    }
}

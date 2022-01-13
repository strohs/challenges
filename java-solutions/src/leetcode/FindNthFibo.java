package leetcode;

public class FindNthFibo {

    public static long nth(int n) {
        if (n == 0) throw new RuntimeException("n must be > 0");
        if (n == 1 || n == 2) return 1;
        long c = 1;
        long p = 1;
        for (int i = 1; i < n - 1; i++) {
            long sum = c + p;
            p = c;
            c = sum;
        }
        return c;
    }

    public static void main(String[] args) {
        System.out.println( String.format("nth of %d = %d", 10, nth(10)));
        System.out.println( String.format("nth of %d = %d", 95, nth(90)));
    }
}

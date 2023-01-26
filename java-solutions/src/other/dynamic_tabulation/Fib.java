package other.dynamic_tabulation;

public class Fib {

    // calculate the nth fibonacci number using tabulation rather than recursion
    // n = nth fib number to compute
    // run time: O(n)
    // space: O(n)
    static long fib(int n) {
        long [] tab = new long[n + 2];
        tab[0] = 0;
        tab[1] = 1;

        for (int i = 2; i <= n; i++) {
            tab[i] = tab[i-1] + tab[i-2];
        }
        return tab[n];
    }


    public static void main(String[] args) {
        System.out.println(fib(6)); // 8
        System.out.println(fib(20)); // 6765
        System.out.println(fib(30)); // 832040
        System.out.println(fib(0));
    }
}

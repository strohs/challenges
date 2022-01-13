package hackerrank.algos.easy;

import java.math.BigInteger;

public class ExtraLongFactorial {

    static void extraLongFactorials(int n) {
        BigInteger b = BigInteger.ONE;
        for (int i = n; i >= 1; i--) {
            b = b.multiply( BigInteger.valueOf(i) );
        }
        System.out.println(b);
    }

    public static void main(String[] args) {
        extraLongFactorials(25);
    }
}

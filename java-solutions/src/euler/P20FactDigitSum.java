package euler;

import java.math.BigInteger;

/**
 * Problem 20 - Factorial Digit Sum
 * n! means n × (n − 1) × ... × 3 × 2 × 1
 * For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
 * and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
 * Find the sum of the digits in the number 100!
 */
public class P20FactDigitSum {

    public static BigInteger fact(int n) {
        BigInteger bi = new BigInteger("1");
        for ( int i = 1; i <= n; i++ ) {
            bi = bi.multiply(BigInteger.valueOf(i));
        }
        return bi;
    }

    public static void main (String[] args) {
        BigInteger fact100 = fact(100);
        int sum = fact100.toString(10)
                .chars()
                .map( ch -> Character.digit(ch, 10))
                .sum();
        System.out.println("sum is " + sum);
    }
}

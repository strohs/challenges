package euler;

import java.math.BigInteger;

/**
 * Created with IntelliJ IDEA.
 * User: Cliff
 * Date: 1/24/2016
 * Time: 6:55 PM
 */
public class P12 {

    private final static int THRESHOLD = 500;

    public static void main(String[] args) {
        //System.out.println(getTriangle());
        System.out.println( lucas(2) );
        System.out.println( lucas(3) );
        System.out.println( lucas(4) );
        System.out.println( lucas(100) );

    }

    //get nth number in lucas series
    public static BigInteger lucas( int n ) {
        if ( n == 0 ) return BigInteger.valueOf( 2L );
        if ( n == 1 ) return BigInteger.valueOf( 1L );

        BigInteger tNum = BigInteger.valueOf( 1L );
        BigInteger pNum = BigInteger.valueOf( 2L );
        BigInteger temp;
        for ( int count = n; count > 1; count--) {
            temp = tNum;
            tNum = tNum.add( pNum );
            pNum = temp;
        }
        return tNum;
    }

    public static long getTriangle() {
        int n = 1;
        long currentSum = 0;
        while (!hasOverXDivisors(currentSum, THRESHOLD)) {
            currentSum += n;
            n++;
        }
        return currentSum;
    }

    private static boolean hasOverXDivisors(long nr, int threshold) {
        if ( nr <= threshold ) {
            return false;
        }
        int divisors = 0;
        int i;
        int sqrt = (int) Math.sqrt(nr);
        for ( i = 1 ; i <= sqrt ; i++ ) {
            if ( nr % i == 0 ) {
                divisors+=2;           // E.g.: (2, n/2), (3, n/3)
            }
        }
        if ( sqrt*sqrt == nr ){        // it was counted twice
            divisors--;
        }
        return divisors > threshold;
    }

}
package google;/// Count the ways to reach the N'th stair
///  there are N stairs, a person at the bottom wants tp reach the top,  and can climb either 1 or 2 stairs at
/// a time. Count the number of ways to reach the top.
/// # Example
/// N = 1
///  1 way: 1
/// N = 2
///  2 ways:  1,1  2
/// N = 3
///  3 ways:  1,1,1   2,1  1,2
/// N = 4
///  5 ways:  1,1,1,1   1,2,1  1,1,2  2,1,1  2,2


public class RecursiveStaircaseProblem {

    // A simple recursive program to find n'th fibonacci number
    // this is O(2^n)
    static int fib(int n)
    {
        if (n <= 1) {
            return n;
        }
        return fib(n-1) + fib(n-2);
    }

    // Returns number of ways to reach s'th stair
    static int countWays(int s)
    {
        return fib(s + 1);
    }


    /* Driver program to test above function */
    public static void main (String[] args)
    {
        int s = 4;
        System.out.println(String.format("number of ways for n=%d is %d", s, countWays(s)));
    }
}

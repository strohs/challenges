package leetcode;/// # 29. Divide Two Integers
///
/// Given two integers `dividend` and `divisor`, divide two integers without using multiplication,
/// division and mod operator.
/// Return the `quotient` after dividing `dividend` by `divisor`.
/// The integer division should truncate toward zero, which means losing its fractional part.
/// For example, `truncate(8.345) = 8` and `truncate(-2.7335) = -2`.
///
/// ## Note
/// - Both dividend and divisor will be 32-bit signed integers.
/// - The divisor will never be 0.
/// - Assume we are dealing with an environment which could only store integers within the 32-bit
///   signed integer range: `[−2^31,  2^31 − 1]`. For the purpose of this problem, assume that your
///   function returns `2^31 − 1` when the division result overflows.
///
/// ## Example 1
/// Input: dividend = 10, divisor = 3
/// Output: 3
/// Explanation: 10/3 = truncate(3.33333..) = 3.


public class DivideTwoIntegers {

    static int divide(int dividend, int divisor) {
        if (dividend == 0) return 0;

        int sign = -1;
        if ((dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0)) sign = 1;

        int dend = Math.abs(dividend);
        int dsor = Math.abs(divisor);

        int qoutient = 0;
        while (dend >= dsor) {
            qoutient++;
            dend -= dsor;
        }

        return qoutient * sign;
    }

    public static void main (String[] args) {
        assert divide(10, 3) == 3;

        assert divide(-10, 3) == -3;

        assert divide(-10, -3) == 3;

        assert divide(0, 3) == 0;

        assert divide(11, 22) == 0;
    }
}

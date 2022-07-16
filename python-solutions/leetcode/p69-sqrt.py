# Problem 69 - Sqrt(x)
# Given a non-negative integer x, compute and return the square root of x.
# Since the return type is an integer, the decimal digits are truncated, and
# only the integer part of the result is returned.
#
# Note: You are not allowed to use any built-in exponent function or operator,
# such as pow(x, 0.5) or x ** 0.5.


class Solution:
    # maximum possible integer in this challenge is: 2147483647
    def mySqrt(self, x: int) -> int:

        def greatest_divisor(n: int, base: int) -> int:
            if n == 0:
                return 0
            for i in reversed(range(1, 10)):
                temp = base * 10 + i
                if n // (temp * i) != 0:
                    return i
            return 0

        if x < 99:
            return greatest_divisor(x, 0)

        # break x into pairs from right to left
        pairs = []
        while x > 0:
            pairs.insert(0, x % 100)
            x = x // 100

        gd = 0
        rem = 0
        base = 0
        sqrt = ""
        for pair in pairs:
            div = rem * 100 + pair
            base = base * 10 + gd * 2
            gd = greatest_divisor(div, base)
            sqrt += str(gd)
            rem = div - ((base * 10 + gd) * gd)

        return int(sqrt)



if __name__ == "__main__":
    sol = Solution()

    res = sol.mySqrt(1)
    print(res)

    res = sol.mySqrt(4)
    print(res)

    res = sol.mySqrt(8)
    print(res)

    res = sol.mySqrt(50965321)
    print(res)

    res = sol.mySqrt(0)
    print(res)

    res = sol.mySqrt(8192)
    print(res)

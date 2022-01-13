# # Problem 43 - Multiply Two Strings
# Given two non-negative integers `num1` and `num2` represented as strings, return the product of `num1` and `num2`,
# also represented as a string.
#
# Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
#
# ## Example 2
# Input: num1 = "123", num2 = "456"
# Output: "56088"

class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        # returns index of first non-zero digit within the list: ls, othwerwise returns the length of ls
        def first_non_zero_index(ls: list[int]) -> int:
            for idx, digit in enumerate(ls):
                if digit != 0:
                    return idx
            return len(ls)

        # base case checks
        if num1 == "0" or num2 == "0":
            return "0"
        if num1 == "1":
            return num2
        if num2 == "1":
            return num1

        # holds the results of our calculations, will never need to be larger the len(num1) + len(num2)
        res = [0] * (len(num1) + len(num2))
        # m will hold the smaller of num1, or num2
        (m, n) = (num1, num2) if len(num1) < len(num2) else (num2, num1)

        for i, c1 in reversed(list(enumerate(m))):
            for j, c2 in reversed(list(enumerate(n))):
                d1, d2 = int(c1, 10), int(c2, 10)
                res[i + j] += (d1 * d2) // 10
                res[i + j + 1] += (d1 * d2) % 10

                if res[i+j+1] >= 10:
                    quot, rem = res[i+j+1] // 10, res[i+j+1] % 10
                    res[i+j+1] = rem
                    res[i+j] += quot

        nz_index = first_non_zero_index(res)
        return "".join(map(lambda digit: str(digit), res[nz_index:]))


sol = Solution()

print("22 x 66 = {}".format(sol.multiply("22", "66")))
# 56088
print("123 x 456 = {}".format(sol.multiply("123", "456")))

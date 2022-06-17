from typing import List


class Solution:

    def plusOne(self, digits: List[int]) -> List[int]:
        carry = 1
        for i in reversed(range(len(digits))):
            plus1 = digits[i] + carry
            digits[i] = plus1 % 10
            carry = plus1 // 10
        if carry == 1:
            digits.insert(0, 1)
        return digits


sol = Solution()
digits1 = [1, 2, 3]
digits1 = sol.plusOne(digits1)
print(digits1)

digits2 = [9]
digits2 = sol.plusOne(digits2)
print(digits2)

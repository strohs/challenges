# Problem 59 -  Spiral Matrix 2
# Given a positive integer n, generate an n x n matrix
# filled with elements from 1 to n2 in spiral order.
#
# `1 <= n <= 20`
#
from typing import List
from itertools import cycle


class Solution:
    def generateMatrix(self, n: int) -> List[List[int]]:
        matrix = [[0] * n for i in range(n)]
        rb, re, cb, ce = 0, n, 0, n
        cur = 1

        for i in cycle([1, 2, 3, 4]):
            if i % 4 == 1:
                for c in range(cb, ce):
                    matrix[rb][c] = cur
                    cur += 1
                rb += 1

            if i % 4 == 2:
                for r in range(rb, re):
                    matrix[r][ce - 1] = cur
                    cur += 1
                ce -= 1

            if i % 4 == 3:
                for c in reversed(range(cb, ce)):
                    matrix[re - 1][c] = cur
                    cur += 1
                re -= 1

            if i % 4 == 0:
                for r in reversed(range(rb, re)):
                    matrix[r][cb] = cur
                    cur += 1
                cb += 1

            if rb >= re or cb >= ce:
                break

        return matrix


sol = Solution()
print(sol.generateMatrix(4))

# Problem 74 - Search a 2D Matrix
# Write an efficient algorithm that searches for a value target in an m x n integer
# matrix `matrix`. This matrix has the following properties:
# - Integers in each row are sorted from left to right.
# - The first integer of each row is greater than the last integer of the previous row.
from typing import List
import bisect


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        row = bisect.bisect_left(matrix, target, key=lambda r: r[0])
        if row > 0:
            row -= 1
        if matrix[row][0] == target:
            return True

        col = bisect.bisect_left(matrix[row], target)
        if col == len(matrix[row]):
            return False
        else:
            return matrix[row][col] == target




sol = Solution()
mat = [[1, 3, 5, 7], [10, 11, 16, 18], [20, 23, 26, 29], [30, 33, 36, 39], [40, 42, 44, 48]]
print(sol.searchMatrix(mat, 11))
print(sol.searchMatrix(mat, 0))
print(sol.searchMatrix(mat, 50))
print(sol.searchMatrix(mat, 48))



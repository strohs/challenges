# Problem 73 - Set Matrix Zeroes
# Given an m x n integer matrix, if an element is 0, set its entire row and column to 0's.
#
# You must do it in place.
from typing import List


class Solution:

    @staticmethod
    def set_zeroes(matrix: List[List[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """

        # check if the 0th row and 0th column have any natural zeroes in them
        # if so, they will need to be set to all zeroes at the end of the algorithm
        zero_row = 0 in matrix[0]
        zero_col = 0 in [row[0] for row in matrix]

        # use the top row and leftmost column to mark which rows/cols have a zero in them
        for ri in range(1, len(matrix)):
            for ci in range(1, len(matrix[0])):
                if matrix[ri][ci] == 0:
                    matrix[0][ci] = 0
                    matrix[ri][0] = 0

        for ci in range(1, len(matrix[0])):
            if matrix[0][ci] == 0:
                for ri in range(1, len(matrix)):
                    matrix[ri][ci] = 0

        for ri in range(1, len(matrix)):
            if matrix[ri][0] == 0:
                for ci in range(1, len(matrix[0])):
                    matrix[ri][ci] = 0

        if zero_row:
            for i in range(len(matrix[0])):
                matrix[0][i] = 0

        if zero_col:
            for i in range(len(matrix)):
                matrix[i][0] = 0


if __name__ == "__main__":
    m1 = [
        [0,1,2,0],
        [3,4,5,2],
        [1,3,1,5],
    ]
    res = [
        [0,0,0,0],
        [0,4,5,0],
        [0,3,1,0],
    ]
    sol = Solution()
    Solution.set_zeroes(m1)
    assert m1 == res

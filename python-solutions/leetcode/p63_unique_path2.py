from typing import List


class Solution:

    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        """
        # Problem 63 - Unique Paths 2
        You are given an m x n integer array grid. There is a robot initially located at
        the top-left corner (i.e., `grid[0][0]`). The robot tries to move to the bottom-right
        corner (i.e., `grid[m-1][n-1]`). The robot can only move either down or right at any
        point in time.
        An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot
        takes cannot include any square that is an obstacle.
        Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
        """
        is_obstacle = lambda row, col: obstacleGrid[row][col] == 1
        row_len = len(obstacleGrid)
        col_len = len(obstacleGrid[0])
        paths = [[0 for _ in obstacleGrid[row]] for row in range(row_len)]

        for c in range(col_len):
            if not is_obstacle(0, c):
                if c == 0:
                    paths[0][c] = 1
                else:
                    paths[0][c] = paths[0][c - 1]

        for r in range(row_len):
            if not is_obstacle(r, 0):
                if r == 0:
                    paths[r][0] = 1
                else:
                    paths[r][0] = paths[r - 1][0]

        for r in range(1, row_len):
            for c in range(1, col_len):
                if is_obstacle(r, c):
                    paths[r][c] = 0
                else:
                    paths[r][c] = paths[r - 1][c] + paths[r][c - 1]

        return paths[row_len - 1][col_len - 1]


grid = [
    [0, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0],
]
sol = Solution()
count = sol.uniquePathsWithObstacles(grid)
print(count)

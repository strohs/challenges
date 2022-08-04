# Problem 70 - Climbing Stairs
# You are climbing a staircase. It takes n steps to reach the top.
#
# Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

class Solution:
    def climbStairs(self, n: int) -> int:
        if n < 2:
            return n
        res = [1, 2]
        for i in range(2, n):
            res.append(res[i-2] + res[i-1])

        return res[n-1]


sol = Solution()
ans = sol.climbStairs(45)
print(ans)


package leetcode.Problem322;

import java.util.Arrays;

/**
 * Problem 322 - Coin change
 * <a href="https://leetcode.com/problems/coin-change/">Coin Change</a>
 */
public class Solution {

    public int coinChange(int[] coins, int amount) {
        // use dynamic programming to compute the fewest number of coins needed to sum to the amount
        int[] dp = new int[amount + 1];
        Arrays.fill(dp, amount + 1); // amount +1 is a sufficient MAX amount, could also use Integer.MAX
        dp[0] = 0;

        // fill the dp array from 1..amount+1
        // for an amount, 'i', what's the fewest number of coins needed to sum to that amount
        for (int amt = 1; amt < amount + 1; amt++) {
            for (int c : coins) {
                if (amt - c >= 0) {
                    // compute the fewest number of coins needed to sum to the current amount, amt,
                    // this sum is 1 (because we selected 'c' itself) + the number of ways to select in dp[amt-c]
                    dp[amt] = Integer.min(dp[amt], 1 + dp[amt - c]);
                }
            }
        }
        return dp[amount] < amount + 1 ? dp[amount] : -1;
    }

    public static void main(String[] args) {

    }
}

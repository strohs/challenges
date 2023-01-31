package other.dynamic_memoization;


import java.time.Duration;
import java.time.Instant;

/**
 * count the number of ways that the coins in the coin array can be combined to form the
 * target amount
 */
public class CoinChange {

    // uses brute_force_recursion. To prevent recounting combinations a subset of the target coins
    // will be passed to different branches
    int brute_force(int target, int[] coins) {
        return brute_force_helper(target, coins, 0);
    }

    // the index 'i' marks the starting index of the coins we can consider within coins.
    // We do this so that we don't count a combination more than once, i.e. 1,1,2 and 2,1,1
    int brute_force_helper(int target, int[] coins, int i) {
        if (target == 0) {
            return 1;
        }
        if (target < 0) {
            return 0;
        }

        int ways_count = 0;

        for (int j = i; j < coins.length; j++) {
            ways_count += brute_force_helper(target - coins[j], coins, j);
        }

        return ways_count;
    }



    int dp_table(int target, int[] coins) {
        int[][] dp = new int[coins.length + 1][target + 1];
        // seed the first column with ones
        for (int i = 1; i < dp.length; i++) {
            dp[i][0] = 1;
        }

        for (int r = 1; r < dp.length; r++) {
            for (int c = 1; c < dp[0].length; c++) {
                int coinAmt = coins[r - 1];
                if (c - coinAmt >= 0) {
                    dp[r][c] += dp[r][c - coinAmt];
                }
                dp[r][c] += dp[r-1][c];
            }
        }
        return dp[coins.length][target];
    }

    public static void main(String[] args) {
        CoinChange sol = new CoinChange();

        int ways = sol.brute_force(5, new int[] {1, 2, 5});
        System.out.println(ways); // 4

        System.out.println(sol.dp_table(5, new int[] {1, 2, 5})); // 4

        System.out.println(sol.dp_table(32, new int[] {2, 4, 8})); // 25

        Instant start = Instant.now();
        System.out.println(sol.brute_force(1000, new int[] {1 ,2, 4, 6, 8}));
        Instant end = Instant.now();
        System.out.println(Duration.between(start, end).toMillis());
    }
}

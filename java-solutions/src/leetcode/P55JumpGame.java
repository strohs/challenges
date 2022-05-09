package leetcode;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

/**
 * # Leetcode Problem 55 - Jump Game
 * You are given an integer array `nums`. You are initially positioned at the array's
 * first index, and each element in the array represents your maximum jump length at that position.
 * Return `true` if you can reach the last index, or `false` otherwise.
 * In this implementation, start at the end of the array and repeatedly find all indices that
 * can jump to the last position of the array
 */
public class P55JumpGame {
    public boolean canJump(int[] nums) {
        for (int i = nums.length - 1; i >= 1; i--) {
            boolean canStillJump = false;
            for (int j = i - 1; j >= 0; j--) {
                if (nums[j] + j >= i) {
                    canStillJump = true;
                    break;
                }
            }
            if (!canStillJump) {
                return false;
            }
        }
        return true;
    }

    /*
        returns the indices of nums that can jump to the last element of nums
     */
    private List<Integer> canReachLast(int[] nums) {
        return IntStream.range(0, nums.length - 1)
                .filter(idx -> idx + nums[idx] >= nums.length - 1)
                .boxed()
                .collect(Collectors.toList());
    }

    /**
     * recursive solution works, but takes too long on leetcode site
     */
    public boolean canJumpRecursive(int[] nums) {
        switch (nums.length) {
            case 0: return false;
            case 1: return true;
            default: {
                int[] jumpIndices = IntStream
                        .range(0, nums.length - 1)
                        .filter(idx -> idx + nums[idx] >= nums.length - 1)
                        .toArray();

                for (int idx: jumpIndices) {
                    if (canJump(Arrays.copyOfRange(nums, 0, idx + 1))) {
                        return true;
                    }
                }
                return false;
            }
        }
    }

    public static void main(String[] args) {
        P55JumpGame jumpGame = new P55JumpGame();

        int[] nums = new int[] { 2,3,1,1,4 };
        System.out.println(jumpGame.canJump(nums));

        nums = new int[] { 1 };
        System.out.println(jumpGame.canJump(nums));

        nums = new int[] { 1,2,3 };
        System.out.println(jumpGame.canJump(nums));

        nums = new int[] { 0 };
        System.out.println(jumpGame.canJump(nums));

        nums = new int[] { 3,2,1,0,4 };
        System.out.println(jumpGame.canJump(nums));

        nums = new int[] { 1,0,1,0 };
        System.out.println(jumpGame.canJump(nums));

        nums = new int[] { 0,2,3 };
        System.out.println(jumpGame.canJump(nums));
    }
}

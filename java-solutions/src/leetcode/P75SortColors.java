package leetcode;

import java.util.Arrays;

/**
 * # 75. Sort Colors
 * Given an array `nums` with `n` objects colored red, white, or blue, sort them in-place so that
 * objects of the same color are adjacent, with the colors in the order red, white, and blue.
 * We will use the integers `0`, `1`, and `2` to represent the color red, white, and blue, respectively.
 * You must solve this problem without using the library's sort function.
 */
public class P75SortColors {

    // swap elements at indices i,j in arr
    private void swap(int [] arr, int i, int j) {
        int t = arr[i];
        arr[i] = arr[j];
        arr[j] = t;
    }

    public void sortColors(int[] nums) {
        // current head of nums
        int h = 0;
        // current tail of nums
        int t = nums.length - 1;
        // index of current element being examined
        int i = 0;

        while (h < t && i <= t) {
            if (nums[i] == 0) {
                swap(nums, h, i);
                if (nums[h] == 0) {
                    h += 1;
                    i += 1;
                }
            } else if (nums[i] == 2) {
                swap(nums, t, i);
                t -= 1;
            } else {
                i += 1;
            }
        }
    }

    public static void main(String[] args) {
        P75SortColors sol = new P75SortColors();
        int [] nums = {2, 0, 2, 1, 1, 0};
        int [] sorted = {0, 0, 1, 1, 2, 2};
        sol.sortColors(nums);

        assert Arrays.equals(nums, sorted);
    }
}

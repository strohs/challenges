package leetcode;

import java.util.*;

/**
 * Given an array nums of distinct integers, return all the possible permutations.
 * You can return the answer in any order.
 *
 */
public class P46Permutations {

    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> results = new ArrayList<>();
        recursivePermute(nums, nums.length, 0, results);
        return results;
    }

    // swap elements at indices i,j in the given array
    private void swap(int[] arr, int i, int j) {
        int t = arr[i];
        arr[i] = arr[j];
        arr[j] = t;
    }

    // generate the permutations of the array nums, from element i to element n-1
    private void recursivePermute(int[] nums, int n, int i, List<List<Integer>> results) {
        // at the end of the current array, so store the current permutation
        if (i == n) {
            List<Integer> perm = new ArrayList<>();
            for (int j = 0; j < n; j++) {
                perm.add(nums[j]);
            }
            results.add(perm);
        } else {
            for (int j = i; j < n; j++) {
                swap(nums, i, j);
                System.out.println("i:" + i + " j:" + j + " " + Arrays.toString(nums));
                recursivePermute(nums, n, i+1, results);
                // swap back
                swap(nums, i, j);
            }
        }
    }

    public static void main(String[] args) {
        P46Permutations p = new P46Permutations();
        int[] ex1 = new int[] {1, 2, 3};

        List<List<Integer>> perms = p.permute(ex1);
        for (List<Integer> perm : perms) {
            System.out.println(perm);
        }
    }
}

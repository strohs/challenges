package leetcode;

import java.util.ArrayList;
import java.util.List;

/** # 39. Combination Sum
* Given an array of distinct integers candidates and a target integer target, return a list of
* all unique combinations of candidates where the chosen numbers sum to target. You may return
* the combinations in any order.
*
* The same number may be chosen from candidates an unlimited number of times. Two combinations
* are unique if the frequency of at least one of the chosen numbers is different.
* It is guaranteed that the number of unique combinations that sum up to target is less than
* 150 combinations for the given input.
*
* ## Example 1
* `Input: candidates = [2,3,6,7], target = 7`
* `Output: [[2,2,3],[7]]`
*
* ## Note:
* - candidates will always have at least one element
* - all elements of candidates will be distinct
 **/
class P39CombinationSum {
    public List<List<Integer>> combinationSum(int[] candidates, int target) {
        List<List<Integer>> res = new ArrayList<>();
        List<Integer> prev = new ArrayList<>();
        solver(candidates, prev, target, res);
        return res;
    }

    private void solver(int[] candidates, List<Integer> prev, int target, List<List<Integer>> res) {
        if (target == 0) {
            res.add(prev);
            return;
        }
        int last = !prev.isEmpty() ? prev.get(prev.size() - 1) : Integer.MIN_VALUE;
        for (int num: candidates) {
            if (num >= last && target - num >= 0) {
                List<Integer> nums = new ArrayList<>(prev);
                nums.add(num);
                solver(candidates, nums, target - num, res);
            }
        }
    }

    public static void main(String[] args) {
        P39CombinationSum p39 = new P39CombinationSum();
        int [] cands = new int[] {1,2,3};
        int target = 6;

        List<List<Integer>> res = p39.combinationSum(cands, target);
        res.forEach(System.out::println);
    }
}

package leetcode;/// Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?
/// Find all unique triplets in the array which gives the sum of zero.
///
/// ## Note:
/// The solution set must not contain duplicate triplets.
///
/// ## Example
/// ```
/// Given array nums = [-1, 0, 1, 2, -1, -4],
///
/// A solution set is:
/// [
///   [-1, 0, 1],
///   [-1, -1, 2]
/// ]
/// ```


import java.util.*;

public class ThreeSum {

    public static Set<List<Integer>> threeSum(List<Integer> nums, int sum) {
        Set<List<Integer>> triplets = new HashSet<>();
        for ( int i = 0; i < nums.size(); i++ ) {
            int target = sum - nums.get(i);
            List<List<Integer>> ts = twoSum(nums.subList(i + 1, nums.size()), target);
            for ( List<Integer> intPair : ts ) {
                intPair.add(nums.get(i));
                intPair.sort(Comparator.naturalOrder());
                triplets.add(intPair);
            }
        }
        return triplets;
    }

    public static List<List<Integer>> twoSum(List<Integer> nums, int sum) {
        Map<Integer,Integer> numMap = new HashMap<>();
        List<List<Integer>> pairs = new ArrayList<>();
        for ( Integer num : nums ) {
            int targetSum = sum - num;
            if (numMap.containsKey(targetSum)) {
                List<Integer> pair = new ArrayList<Integer>(Arrays.asList(num, targetSum));
                pairs.add(pair);
            }
            numMap.putIfAbsent(num, num);
        }
        return pairs;
    }

    public static void main (String[] args) {
        List<Integer> input1 = Arrays.asList(-1, 0, 1, 2, -1, -4);
        Set<List<Integer>> res1 = threeSum(input1, 0);
        System.out.println(res1); // [ [-1, -1, 2], [-1, 0, 1] ]

        List<Integer> input2 = Arrays.asList(1,2,1,2,1,2,1);
        Set<List<Integer>> res2 = threeSum(input2, 6);
        System.out.println(res2); // [ [2, 2, 2] ]

    }
}

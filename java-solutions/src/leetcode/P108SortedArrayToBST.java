package leetcode;

import java.util.Arrays;

/**
 * Leetcode 108 - Convert Sorted Array to Binary Search Tree
 * https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
 */
public class P108SortedArrayToBST {

    private class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {}

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }


    class Solution {
        public TreeNode sortedArrayToBST(int[] nums) {
            return helper(nums, null);
        }

        private TreeNode helper(int[] nums, TreeNode node) {
            if (nums.length == 0) {
                return null;
            } else {
                int mid = nums.length / 2;
                node = new TreeNode(nums[mid]);
                node.left = helper(Arrays.copyOfRange(nums, 0, mid), node.left);
                node.right = helper(Arrays.copyOfRange(nums, mid+1, nums.length), node.right);
                return node;
            }
        }
    }
}

package leetcode;

/**
 * Problem 112 - Path Sum
 * https://leetcode.com/problems/path-sum/
 *
 * The key for this challenge was to remember that: "A leaf is a node with no children"
 */
public class P112PathSum {

    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    public boolean hasPathSum(TreeNode root, int targetSum) {
        if (root == null) {
            return false;
        } else {
            if (root.left == null && root.right == null) {
                return targetSum - root.val == 0;
            } else {
                return hasPathSum(root.left, targetSum - root.val) || hasPathSum(root.right, targetSum - root.val);
            }
        }
    }
}

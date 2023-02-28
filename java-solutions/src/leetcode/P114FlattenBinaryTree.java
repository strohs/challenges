package leetcode;

/**
 * Leetcode Problem 114 - Flatten Binary Tree to linked list
 * https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
 */
public class P114FlattenBinaryTree {

    private static class TreeNode {
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

    /**
     * returns the rightmost node of the given node, if there is no right node then the given node
     * is returned
     */
    private TreeNode getRightmost(TreeNode node) {
        TreeNode cur = node;
        while (cur.right != null) {
            cur = cur.right;
        }
        return cur;
    }

    public void flatten(TreeNode root) {
        TreeNode cur = root;
        while (cur != null) {
            // check if left branch exists
            if (cur.left != null) {
                TreeNode rightmost = getRightmost(cur.left);
                rightmost.right = cur.right;
                cur.right = cur.left;
                cur.left = null;
            }
            cur = cur.right;
        }

    }
}

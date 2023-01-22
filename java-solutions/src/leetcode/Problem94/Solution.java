package leetcode.Problem94;

import java.util.ArrayList;
import java.util.List;

/**
 * Problem 94 - Binary Tree Inorder Traversal
 * <a href="https://leetcode.com/problems/binary-tree-inorder-traversal/">...</a>
 */
public class Solution {

    //Definition for a binary tree node.
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

    public List<Integer> inorderTraversal(TreeNode root) {
        List<Integer> nodes = new ArrayList<>();
        inorderRec(nodes, root);
        return nodes;
    }

    void inorderRec(List<Integer> nodes, TreeNode node) {
        if (node == null) {
            return;
        }
        inorderRec(nodes, node.left);
        nodes.add(node.val);
        inorderRec(nodes, node.right);
    }

    public static void main(String[] args) {}
}

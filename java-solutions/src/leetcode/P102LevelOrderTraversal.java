package leetcode;

import java.util.*;

public class P102LevelOrderTraversal {

    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode() {}
        TreeNode(int val) { this.val = val; }
        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> res = new ArrayList<>();
        if (root == null) return res;

        Deque<TreeNode> toVisit = new ArrayDeque<>();
        toVisit.add(root);

        while (!toVisit.isEmpty()) {
            Deque<TreeNode> level = new ArrayDeque<>();
            List<Integer> vals = new ArrayList<>();
            while (!toVisit.isEmpty()) {
                TreeNode node = toVisit.removeFirst();
                vals.add(node.val);
                if (node.left != null) level.addLast(node.left);
                if (node.right != null) level.addLast(node.right);
            }
            res.add(vals);
            toVisit.addAll(level);
        }
        return res;
    }

    public static void main(String[] args) {
        P102LevelOrderTraversal sol = new P102LevelOrderTraversal();
        //TreeNode left = new TreeNode(3, new TreeNode(1, null, null), new TreeNode(4, null, null));
        TreeNode left = new TreeNode(3, null, null);
        TreeNode right = new TreeNode(7, new TreeNode(6, null, null), new TreeNode(8, null, null));
        TreeNode root = new TreeNode(5, left, right);

        var res = sol.levelOrder(root); // [5], [3,7], [1,4,6,8]
        for (List<Integer> level: res) {
            System.out.println(level);
        }
    }
}

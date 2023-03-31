package leetcode;

import java.util.ArrayList;

public class P129SumRootToLeafNumbers {

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

    public int sumNumbers(TreeNode root) {
        ArrayList<Integer> res = new ArrayList<>();
        recurse(root, "", res);
        return res.stream().reduce(0, Integer::sum);
    }

    private void recurse(TreeNode node, String cur, ArrayList<Integer> acc) {
        cur = cur + node.val;
        if (node.left == null && node.right == null) {
            acc.add( Integer.parseInt(cur) );
        } else {
            if (node.left != null)
                recurse(node.left, cur, acc);
            if (node.right != null)
                recurse(node.right, cur, acc);
        }
    }

    public static void main(String[] args) {
        TreeNode tree1 = new TreeNode(1);
        tree1.right = new TreeNode(3);
        tree1.left = new TreeNode(2);
        TreeNode tree2 = new TreeNode(4);
        tree2.right = new TreeNode(0);
        tree2.left = new TreeNode(9, new TreeNode(5), new TreeNode(1));

        P129SumRootToLeafNumbers sol = new P129SumRootToLeafNumbers();
        int res = sol.sumNumbers(tree2);
        System.out.println(res);
    }
}

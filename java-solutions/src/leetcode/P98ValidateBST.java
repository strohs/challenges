package leetcode;

public class P98ValidateBST {

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

    TreeNode prev = null;

    public boolean isValidBST(TreeNode root) {
        if (root == null)
            return true;

        if(!isValidBST(root.left))
            return false;

        int pval = this.prev != null ? this.prev.val : -1;
        System.out.println("prev "  + pval + "  root " + root.val);

        if (prev != null && prev.val >= root.val)
            return false;

        prev = root;

        if (!isValidBST(root.right))
            return false;

        return true;

    }


    public static void main(String[] args) {
        P98ValidateBST sol = new P98ValidateBST();
        TreeNode r = new TreeNode(10);
        r.left = new TreeNode(8);
        r.right = new TreeNode(20);
        boolean valid = sol.isValidBST(r);
        System.out.println(valid);
    }
}

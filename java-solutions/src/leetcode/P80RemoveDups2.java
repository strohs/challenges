package leetcode;

public class P80RemoveDups2 {

    public static int removeDuplicates(int[] nums) {
        int i = 0;
        for (int n : nums) {
            if (i < 2 || n > nums[i - 2])
                nums[i++] = n;
        }
        return i;
    }

    public static void main(String[] args) {
        int i = 0;
        int [] nums = { 1, 1, 2, 2, 2, 3, 3, 3, 3 ,4 ,5 };
        int k = P80RemoveDups2.removeDuplicates(nums);
        System.out.println("k = " + k);
    }
}

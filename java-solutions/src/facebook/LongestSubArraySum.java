package facebook;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/// given an array of integers, of some length, N, find the longest sub-array of integers that sum to some
/// value, S.  If there is no such sub-array, return an empty array
/// Ex.  arr = [1,3,5,4,7,5] S=12
///     the longest subarray is [3,5,4]

public class LongestSubArraySum {

    public static List<Integer> findLongestSubArraySum(List<Integer> nums, int s) {
        List<Integer> longest = new ArrayList<>();

        for (int csi = 0; csi < nums.size(); csi++) {
            int i = csi;
            int diff = s;
            while (i < nums.size() && diff >= 0) {
                diff -= nums.get(i);
                if (diff == 0) {
                    if ((i + 1) - csi > longest.size()) {
                        longest = nums.subList(csi, i+1);
                    }
                }
                i += 1;
            }
        }
        return longest;
    }


    public static void main(String[] args) {
        int s = 12;
        List<Integer> a1 = Arrays.asList(1,3,5,4,0,0,8,5,7);
        List<Integer> a2 = Arrays.asList(2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1);
        //List<Integer> a3 = Arrays.asList(12,8,4,3,3,3,3,5,4,3);
        List<Integer> a4 = Arrays.asList(1,2,3,4,5,0,0,0,0,0);
        System.out.println(findLongestSubArraySum(a1, s));
        System.out.println(findLongestSubArraySum(a2, s));
        //System.out.println(findLongestSubArraySum(a3, s));
        System.out.println(findLongestSubArraySum(a4, s));

    }
}

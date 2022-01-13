package google;

import java.util.*;

/**
 * given an array of integers, find the integer that occurs the most times withing the array.
 *
 */
public class GreatestIntegerCount {

    public static int greatestCount(ArrayList<Integer> nums) {
        Map<Integer,Integer> map = new HashMap<>();
        int [] greatest = new int[] { nums.get(0), 0 };
        for (int n : nums) {
            map.merge(n, 1, Integer::sum);
            if (map.get(n) > greatest[1]) {
                greatest[0] = n;
                greatest[1] = map.get(n);
            }
        }

        // sort the map entries by value, descending, into a list of Map,Entry
//        List<Map.Entry<Integer, Integer>> list = new ArrayList<>(map.entrySet());
//        list.sort(Map.Entry.comparingByValue(Comparator.reverseOrder()));

        return greatest[0];
    }

    public static void main(String[] args) {
        ArrayList<Integer> nums = new ArrayList<>(Arrays.asList(1, 2, 3, 4, 4, 5, 6, 7,8,8, 8, 9));
        int gc = greatestCount(nums);
        System.out.println(gc);
    }
}

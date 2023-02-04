package leetcode.Problem658;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class FindKClosest {

    // Strategy
    // binsearch arr for x
    //  if binsearch == -1, x is outside left edge, return arr[0..k]
    //  if binsearch.abs == arr.length + 1, then x outside right edge, return arr[length - k .. arr.length]
    //  else if x is found, or binsearch returns some other negative value, binsearch (result + 1).abs,
    //    need to set l,r pointers, start l at insertion index and right at l + 1, need to also bounds check indices

    public List<Integer> findClosestElements(int[] arr, int k, int x) {
        List<Integer> closest = new ArrayList<>();

        if (arr.length <= k) {
            return Arrays.stream(arr).boxed().collect(Collectors.toList());
        }

        // try to find starting index of x if it exists
        int t = Arrays.binarySearch(arr, x);
        if (t == -1) {
            for (int i = 0; i < k; i++) {
                closest.add(arr[i]);
            }
        } else if (Math.abs(t) == arr.length + 1) {
            for (int i = arr.length - k; i < arr.length; i++) {
                closest.add(arr[i]);
            }
        } else {
            int l = 0, r;
            if ( t < 0 ) {
                l = Math.abs(t+2);
            } else {
                l = t;
            }
            r = l + 1;

            while (k > 0) {
                if (l >= 0 && r >= arr.length) {
                    // can only move l
                    l--;
                } else if (l < 0 && r < arr.length) {
                    // can only move r
                    r++;
                } else {
                    // both l and r still inbounds
                    int a = Math.abs(arr[l] - x);
                    int b = Math.abs(arr[r] - x);
                    if (a <= b) {
                        l--;
                    } else {
                        r++;
                    }
                }
                k--;
            }
            for (int i = l+1; i < r; i++) {
                closest.add(arr[i]);
            }
        }

        return closest;
    }

    public static void main(String[] args) {
        FindKClosest sol = new FindKClosest();

        System.out.println(sol.findClosestElements(new int[] { 1, 2 ,3 ,4 ,5 }, 4, 3)); // [1,2,4,6]

        System.out.println(sol.findClosestElements(new int[] { 1, 2 ,3 ,4 ,5 }, 4, -1));

        System.out.println( sol.findClosestElements(new int[] {3}, 1, 3) ); // 3

        System.out.println( sol.findClosestElements(new int[] {3}, 1, 5) ); // 3

        System.out.println( sol.findClosestElements(new int[] {3}, 1, 1) ); // 3

        System.out.println( sol.findClosestElements(new int[] {3, 4 ,5}, 2, 3) ); // 3,4

        System.out.println( sol.findClosestElements(new int[] {3, 4 ,5}, 2, 5) ); // 4,5

        System.out.println( sol.findClosestElements(new int[] {3, 4 ,5}, 2, 8) ); // 4,5

        System.out.println(sol.findClosestElements(new int[] { 0,0,1,2,3,3,4,7,7,8 }, 3, 5)); //3,3,4

        System.out.println(sol.findClosestElements(new int[] { 1,2,5,5,6,6,7,7,8,9 }, 7, 7)); // 5,5,6,6,7,7,8
    }
}

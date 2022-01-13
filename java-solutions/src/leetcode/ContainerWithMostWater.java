package leetcode;
/*

# 11. Container with the most water
Given `n` non-negative integers `a1, a2, ..., an` , where each represents a point at
coordinate (i, ai). `n` vertical lines are drawn such that the two endpoints of line i is
at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such
that the container contains the most water.
**NOTE**: You may not slant the container and n is at least 2.
## Example
`Input: [1, 8, 6, 2, 5, 4, 8, 3, 7]`
`Output: 49`
 */

import java.util.function.BiFunction;

public class ContainerWithMostWater {

    static private int xLength(int cur_pos, int start_pos) {
        return cur_pos - start_pos;
    }

    // this method does the same thing as xLength
    static BiFunction<Integer, Integer, Integer> xDistance = (c, s) -> c - s;

    static int maxArea(int [] arr) {
        int maxArea = 0;

        for (int i = 0; i < arr.length; i++) {
            for (int j = i+1; j < arr.length; j++) {
                int curArea = 0;
                if (arr[i] < arr[j]) {
                    //curArea = xDistance.apply(j, i);
                    curArea = arr[i] * xLength(j, i);
                } else {
                    curArea = arr[j] * xLength(j, i);
                }

                if (curArea > maxArea) {
                    System.out.printf("old max: %d  new max: %d  at index:(%d, %d)\n", maxArea, curArea, i, j);
                    maxArea = curArea;
                }
            }
        }

        return maxArea;
    }

    public static void main(String[] args) {
        int [] arr = {1, 8, 6, 2, 5, 4, 8, 3, 7};
        System.out.println(maxArea(arr));   //49

        int [] arr2 = {1, 2, 2, 1};
        System.out.println(maxArea(arr2));  //3
    }
}

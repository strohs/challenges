package freecodecamp;

import java.util.Arrays;

/**
 * Find the first and last index of an element, t,  within a sorted array, arr.
 * Return an array containing the first/last indices of the element or if the elements is not found
 * return [-1, -1]
 * assume the input array will not be empty
 */
public class FirstLastIndex {

    // returns the midpoint between indices bi and ei. bi must be <= ei
    int midindex(int bi, int ei) {
        if (ei - bi == 0) {
            return 0;
        } else {
            return ((ei - bi) / 2) + 1 + bi;
        }
    }

    // return index of element in the sorted arr that equals t. If t not found, return -1
    public int binarySearch(int[] arr, int bi, int ei, int t) {
        if (arr[bi] == t) {
            return bi;
        } else {
            int mi = midindex(bi, ei);
            if (mi >= arr.length || mi <= 0) {
                return -1;
            }
            if (arr[mi] == t) {
                return mi;
            } else if (arr[mi] < t) {
                bi = mi + 1;
                if (bi >= arr.length) {
                    return -1;
                } else {
                    return binarySearch(arr, bi, ei, t);
                }
            } else {
                // arr[mi] > t
                ei = mi - 1;
                return binarySearch(arr, bi, ei, t);
            }
        }
    }

    // find the last index of t within arr, using index bi as the leftmost bound
    public int probeRight(int[] arr, int bi, int ei, int t) {
        if (arr[bi] == arr[ei]) {
            return ei;
        } else {
            // find mid-point index
            int mi = midindex(bi, ei);
            if (arr[mi] == t) {
                bi = mi;
                ei -= 1;
            } else {
                ei = mi - 1;
            }
            return probeRight(arr, bi, ei, t);
        }
    }

    // find the first occurrence of t within arr, using ei as the rightmost bound of the sub-array
    public int probeLeft(int[] arr, int bi, int ei, int t) {
        if (arr[bi] == arr[ei]) {
            return bi;
        } else {
            // find mid-point index
            int mi = midindex(bi, ei);
            if (arr[mi] == t) {
                ei = mi;
                bi += 1;
            } else {
                bi = mi + 1;
            }
            return probeLeft(arr, bi, ei, t);
        }
    }

    public int[] searchRange(int[] nums, int t) {
        // general idea is to binary search consecutively smaller slices of the array, keeping track
        // of the current begin index (bi), end index (ei) and midpoint index (mi) of the array.
        // When arr[mi] == t, we recursively binary search the left, right halves of the array until
        // We stop when arr[bi] == arr[ei] == t, OR when bi or ei go past the bounds of the array
        // and no element == t is found
        int bi = 0;
        int ei = nums.length - 1;
        int[] res = new int[]{-1, -1};

        if (nums.length == 0) {
            return res;
        }

        // binary search for the element t
        int i = binarySearch(nums, bi, ei, t);
        if (i == -1) {
            // element t not found within arr
            return res;
        } else {
            // element found, probe left and probe right to get leftmost and rightmost position of t
            res[0] = probeLeft(nums, bi, i, t);
            res[1] = probeRight(nums, i, ei, t);
            return res;
        }

    }

    public static void main(String[] args) {
        var fli = new FirstLastIndex();
        int[] arr1 = {1, 2, 3, 4, 5, 6, 7};
        int[] arr2 = {1, 2, 3, 4, 5, 6, 7, 8};
        int[] arr3 = {1, 2, 5, 5, 5, 6, 7};
        int[] arr4 = {5, 7, 7, 8, 8, 10};
        int[] arr5 = {2 ,2};

        int[] res = fli.searchRange(arr5, 1);
        System.out.println("first/last indices are " + Arrays.toString(res));
    }
}

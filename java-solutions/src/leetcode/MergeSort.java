package leetcode;

import java.util.Arrays;
import java.util.stream.IntStream;

// MergeSort(arr[], l,  r)
// Time: O(n * log(n))
// If r > l
//     1. Find the middle point to divide the array into two halves:
//             middle m = (l+r)/2
//     2. Call mergeSort for first half:
//             Call mergeSort(arr, l, m)
//     3. Call mergeSort for second half:
//             Call mergeSort(arr, m+1, r)
//     4. Merge the two halves sorted in step 2 and 3:
//             Call merge(arr, l, m, r)

public class MergeSort {

    private int[] slice(int[] arr, int start, int end) {
        return Arrays.stream(arr, start, end)
                .toArray();
    }

    private int[] combine(int[] l, int[] r) {
        int[] marr = new int[l.length + r.length];
        int m = 0;
        int i = 0;
        int j = 0;
        while (i < l.length && j < r.length) {
            if (l[i] <= r[j]) {
                marr[m++] = l[i++];
            } else {
                marr[m++] = r[j++];
            }
        }
        while (i < l.length) {
            marr[m++] = l[i++];
        }
        while (j < r.length) {
            marr[m++] = r[j++];
        }
        return marr;
    }

    public int[] merge_sort(int[] arr) {
        if (arr.length <= 1) {
            return arr;
        }
        int mp = arr.length / 2;
        int[] l = slice(arr, 0, mp);
        int[] r = slice(arr, mp, arr.length);

        // recursively sort both sublists
        l = merge_sort(l);
        r = merge_sort(r);
        return combine(l, r);
    }

    public static void main(String[] args) {
        int[] arr1 = { 9,1,3,8,7,2,7,3,6,2,5,2,4,0,5 };
        int[] sorted1 = { 0,1,2,2,2,3,3,4,5,5,6,7,7,8,9 };
        MergeSort ms = new MergeSort();
        int[] merged = ms.merge_sort(arr1);
        assert merged == sorted1;
        System.out.println(Arrays.toString(merged));
    }
}

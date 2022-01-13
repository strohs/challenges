package leetcode;

import java.util.Arrays;
import java.util.PriorityQueue;

// Given an array of n elements, where each element is at most k away from its
// target position, devise an algorithm that sorts in O(n log k) time. For example,
// let us consider k is 2, an element at index 7 in the sorted array, can be at
// indexes 5, 6, 7, 8, 9 in the given array
// # Example
// ```
// Input : arr[] = {6, 5, 3, 2, 8, 10, 9}
//             k = 3
// Output : arr[] = {2, 3, 5, 6, 8, 9, 10}
// ```
//
public class NearlySortedList {

    // sort the given array, where k indicates how far each element of the
    // array is from its final sorted position
    public static void sortNearlySorted(int [] arr, int k) {
        // using a PriorityQueue as the min-heap. It will provide get operations
        var heap = new PriorityQueue<Integer>(k + 1);

        for (int i = 0; i < arr.length; i++) {
            heap.offer(arr[i]); // O(log n)
            if (i >= k + 1) {
                int min = heap.remove(); // O(log n)
                arr[i - (k+1)] = min;
            }
        }
        int i = arr.length;
        // pop any remaining items on the heap onto the end of the array
        while (!heap.isEmpty()) {
            var item = heap.remove(); // O(log n)
            arr[i - (k+1)] = item;
            i += 1;
        }
    }

    public static void main(String[] args) {
        var arr1 = new int [] { 6,5,3,2,8,8,10,9 };
        NearlySortedList.sortNearlySorted(arr1, 3);
        System.out.println(Arrays.toString(arr1));
    }
}

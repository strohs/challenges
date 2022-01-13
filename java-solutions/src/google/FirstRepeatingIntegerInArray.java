package google;

import java.util.Arrays;

// given an array of length N, find a digit in the array that repeats first
// the input array will only have integers between 1 ... N inclusive
// ex. [ 1, 2, 3, 1, 4, 5 ] ==> 1
// ex. [ 6, 2, 1, 2, 5, 6 ] ==> 2  (notice that 6 repeats, BUT 2 repeats before encountering the second 6)
public class FirstRepeatingIntegerInArray {

    public static int findFirstRepeating(int [] arr) {
        for (int i = 0; i < arr.length; i++) {
            int n = Math.abs(arr[i]);
            if (arr[n - 1] < 0) {
                return n;
            } else {
                arr[n - 1] *= -1;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        int [] arr1 = { 1, 2, 3, 4, 5, 6 };
        int [] arr2 = { 6, 5, 2, 4, 2, 6 };
        System.out.println("arr1 =" + findFirstRepeating(arr1));
        System.out.println("arr2 =" + findFirstRepeating(arr2));
    }
}

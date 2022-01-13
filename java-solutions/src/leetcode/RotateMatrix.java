package leetcode;/// rotate an the values in a square array by 90 "degrees" clockwise
/// The input matrix will  a 2-D array of ints
/// The input array must be rotated in place
///
/// # Example
/// Given:
/// ```
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// ```
///
/// 1. transpose the matrix along the diagonal running from 0,0 to n-1,n-1
/// transpose takes rows and turns them into columns:
/// 1 4 7
/// 2 5 8
/// 3 6 9
///
/// 2. then flip the transposed matrix, horizontally (imagine a vertical line thru column n/2
/// 7 4 1
/// 8 5 2
/// 9 6 3
///

public class RotateMatrix {

    public static void print(int[][] arr) {
        for (int r = 0; r < arr[0].length; r++) {
            for (int c = 0; c < arr[0].length; c++) {
                System.out.print(arr[r][c] + " ");
            }
            System.out.println();
        }
    }
    public static void transpose(int[][] arr) {
        for (int r = 0; r < arr[0].length; r++) {
            for (int c = 0; c < arr[0].length; c++) {
                // only swap indices above the array's diagonal
                if (c > r) {
                    int temp = arr[r][c];
                    arr[r][c] = arr[c][r];
                    arr[c][r] = temp;
                }
            }
        }
    }

    public static void horizontalFlip(int[][] arr) {
        // array dimension
        int dim = arr[0].length;
        for (int r = 0; r < arr[0].length; r++) {
            for (int c = 0; c < arr[0].length; c++) {
                // only need to examine the columns in first half of the array
                if (c < arr[0].length / 2) {
                    int sc = (dim - c - 1);
                    int temp = arr[r][c];
                    arr[r][c] = arr[r][sc];
                    arr[r][sc] = temp;
                }
            }
        }
    }

    public static void rotate(int [][] arr) {
        transpose(arr);
        horizontalFlip(arr);
    }

    public static void main(String[] args) {
        int[][] arr = {
                {1,2,3},
                {4,5,6},
                {7,8,9},
        };
        rotate(arr);
        RotateMatrix.print(arr);

//        int[][] arr2 = {
//                { 1,  2,  3,  4},
//                { 5,  6,  7,  8},
//                { 9, 10, 11, 12},
//                {13, 14, 15, 16},
//        };
//        rotate(arr2);
//        RotateMatrix.print(arr2);
    }
}

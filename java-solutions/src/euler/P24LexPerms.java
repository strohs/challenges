package euler;

import java.util.Arrays;
import java.util.stream.Collectors;

/// # Problem 24 - Lexicographic Permutations
///
/// A permutation is an ordered arrangement of objects. For example, 3124 is one possible
/// permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or
/// alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
///
/// 012 021 102 120 201 210

/// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
public class P24LexPerms {

    // Returns a each element of a as a concatenated String
    static String arrToStr(int [] a) {
        return Arrays.stream(a)
                .mapToObj(Integer::toString)
                .collect(Collectors.joining(""));
    }

    // swap elements in the array: a
    static void swapPos(int[] a, int i, int j) {
        int t = a[i];
        a[i] = a[j];
        a[j] = t;
    }

    // find the index of the smallest element in `a` that is to the right of > first
    static int smallestPos(int [] a, int first, int si) {
        int ci = si;
        for (int i = si +1; i < a.length; i++) {
            if (a[i] > first && a[i] < a[ci]) {
                ci = i;
            }
        }
        return ci;
    }

    static String lexPerms(int nth, int[] a) {
        int size = a.length;
        int n = nth;

        while (n > 1) {
            System.out.println(arrToStr(a));

            // find rightmost char which is smaller than its next char, call it 'firstChar'
            int i = 0;
            for (i = size - 2; i >= 0; --i) {
                if (a[i] < a[i+1])
                    break;
            }

            if (i < 0) {
                n = 0;
            } else {
                int ceilIdx = smallestPos(a, a[i], i+1);
                swapPos(a, i, ceilIdx);
                Arrays.sort(a, i+1, size);
                n -= 1;
            }
        }

        return arrToStr(a);
    }

    public static void main(String[] args) {
        int [] a = {0,1,2,3,4,5,6,7,8,9};


        String res = lexPerms(1_000_000, a); //2783915460
        System.out.println(res);
    }
}

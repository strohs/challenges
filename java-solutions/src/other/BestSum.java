package other;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

/**
 * DP practice problem
 * write a function: int[] bestSum(int targetSum, int[] numbers) that returns the shortest combination of numbers
 * that add up to the target sum. If there is no combination at all return null, if there is a tie for shortest, pick
 * one
 * Example:
 * howSum(7, [5, 3, 4, 7]) -> [7]
 * howSum(8, [2, 3, 5]) -> [2,2,2,2] , [5,3] shortest is [5,3]
 * <p>
 * we will need to try all combinations
 * <p>
 * Big O:
 * m = target sum
 * n = numbers.length
 * Brute Force time = O(n^m) * m) = n is the branching factor, up to m levels deep,  the extra '* m' is because we are copying
 *   array that will at worst have m numbers in it
 * Brute Force space = O(m * m) = O(m^2)  m is the max frames on the call stack. m^2 because we are keeping track of the shortestCombo array at each
 *   recursive call, which at worst will be of length m, therefore m * m = m^2
 * <p>
 * Memo time = O(n * m * m) = O(n * m^2) the length of the largest branch we would need to travel at least once, plus we
 *   create and copy a new array at each call, the worse case would be an array of size m
 * Memo space = O(m^2 + m * m) the size of the largest amount of recursive calls on the stack, plus we now store the memo map which
 *   will store at worst m keys containing (at worst) m elements. The memo size dominates the stack calls so we simplify to O(m^2)
 */
public class BestSum {

    int[] bestSum(int targetSum, int[] numbers) {
        HashMap<Integer, int[]> dp = new HashMap<>();
        int [] best = helper(targetSum, numbers, dp);
        return best;
    }

    int[] helper(int targetSum, int[] numbers, HashMap<Integer, int[]> dp) {
        if (dp.containsKey(targetSum)) {
            return dp.get(targetSum);
        }
        if (targetSum == 0) {
            return new int [0];
        }
        if (targetSum < 0) {
            return null;
        }

        int[] shortestCombo = null; // track the current shortest across branches
        for (int num : numbers) {
            int remainder = targetSum - num;
            int[] shortestWay = helper(remainder, numbers, dp);
            if (shortestWay != null) {
                int[] currCombo = concatInt(shortestWay, num);
                if (shortestCombo == null || currCombo.length < shortestCombo.length) {
                    shortestCombo = currCombo;
                }
            }
        }

        dp.put(targetSum, shortestCombo);
        return shortestCombo;
    }

    // returns a new array containing the concatenatation of array2 onto the end of array1.
    static int[] concatArrays(int[] array1, int[] array2) {
        return IntStream.concat(Arrays.stream(array1), Arrays.stream(array2)).toArray();
    }

    // concat an integer onto the end of array1 and return a new array
    static int[] concatInt(int[] array1, int num) {
        int[] array2 = { num };
        return IntStream.concat(Arrays.stream(array1), Arrays.stream(array2)).toArray();
    }

    public static void main(String[] args) {
        BestSum bs = new BestSum();
        int[] numbers = {5, 3, 4, 7};
        int[] ans = bs.bestSum(7, numbers);
        System.out.println(Arrays.toString(ans));

        numbers = new int[] {2, 3, 5};
        ans = bs.bestSum(8, numbers);
        System.out.println(Arrays.toString(ans));

        numbers = new int[] {3, 3};
        ans = bs.bestSum(7, numbers);
        System.out.println(Arrays.toString(ans));

        numbers = new int[] {1, 2, 5, 25};
        ans = bs.bestSum(100, numbers);
        System.out.println(Arrays.toString(ans)); // [25, 25, 25, 25]
    }
}

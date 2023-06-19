package other.dynamic_memoization;

import java.util.Arrays;
import java.util.HashMap;
import java.util.stream.IntStream;

/**
 * DP practice problem
 * write a function: List<List<Integer>> howSum(int targetSum, int[] numbers) that returns any comination of numbers
 * that add up to the target sum. If there is no combination at all return null
 * Example:
 * howSum(7, [5, 3, 4, 7]) -> [3,4] Or [7]  (either would be correct answer)
 * howSum(8, [2, 3, 5]) -> [2,2,2,2] Or [5,3]
 * <p>
 * we essentially want to stop recursing when the first solution is found
 * <p>
 * Big O:
 * m = target sum
 * n = numbers.length
 * Brute Force time = O(n^m) * m) = n is the branching factor, up to m levels deep,  the extra '* m' is because we are copying
 *   array that will at worst have m numbers in it
 * Brute Force space = O(m)  m is the max frames on the call stack. We could also consider the size of the arrays we create, but since
 *   we return the first one that solves the prob, we can ignore this cost
 * <p>
 * Memo time = O(n * m * m) = O(n * m^2) the length of the largest branch we would need to travel at least once, plus we
 *   create and copy a new array at each call, the worse case would be an array of size m
 * Memo space = O(m + (m * m)) = O(m^2) the size of the largest amount of recursive calls on the stack, plus we now store the memo map which
 *   will store at worst m keys containing (at worst) m elements. The memo size dominates the stack calls to we reduce to O(m^2)
 */
public class HowSum {

    int[] howSum(int targetSum, int[] numbers) {
        HashMap<Integer, int[]> dp = new HashMap<>();
        return helper(targetSum, numbers, dp);
    }

    int[] helper(int targetSum, int[] numbers, HashMap<Integer, int[]> dp) {
        if (dp.containsKey(targetSum)) {
            return dp.get(targetSum);
        }
        if (targetSum == 0) {
            return new int[0];
        }
        if (targetSum < 0) {
            return null;
        }

        for (int num : numbers) {
            int remainder = targetSum - num;
            int[] result = helper(remainder, numbers, dp);
            if (result != null) {
                dp.put(remainder, result);
                return concatInt(result, num);
            }
        }

        dp.put(targetSum, null);
        return null;
    }

    // returns a new array containing the concatenation of array2 onto the end of array1.
    static int[] concatArrays(int[] array1, int[] array2) {
        return IntStream.concat(Arrays.stream(array1), Arrays.stream(array2)).toArray();
    }

    // concat an integer onto the end of array1 and return a new array
    static int[] concatInt(int[] array1, int num) {
        int[] array2 = { num };
        return IntStream.concat(Arrays.stream(array1), Arrays.stream(array2)).toArray();
    }


    public static void main(String[] args) {
        HowSum hs = new HowSum();
        int[] numbers = {5, 3, 4, 7};
        int[] ans = hs.howSum(7, numbers);
        System.out.println(Arrays.toString(ans));

        numbers = new int[] {2,3,5};
        ans = hs.howSum(8, numbers);
        System.out.println(Arrays.toString(ans));

        numbers = new int[] {3, 3};
        ans = hs.howSum(7, numbers);
        System.out.println(Arrays.toString(ans));

        numbers = new int[] {7, 14};
        ans = hs.howSum(300, numbers);
        System.out.println(Arrays.toString(ans));
    }
}

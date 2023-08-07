package other;

/**
 * Print the number of ways to partition a String.
 * This solution is taken from GeeksForGeeks website
 * For string 'abcd' there will be 2^(n-1) i.e. 8 partitions.
 * <p>
 * The crux of the solution lies in the recursion to print all the permutations.
 * maintain two parameters – index of the next character to be processed and the output string so far.
 * We start from index of next character to be processed, append substring formed by unprocessed string
 * to the output string and recurse on remaining string until we process the whole string.
 * <p>
 * Time Complexity is O(2^n)
 * </p>
 */
public class PartitionString {

    // find all combinations of non-overlapping
    // substrings formed by input string str
    static void findCombinations(String str, int index,
                                 String out) {
        if (index == str.length())
            System.out.println(out);

        for (int i = index; i < str.length(); i++)

            // append substring formed by str[index,
            // i] to output string
            findCombinations(str, i + 1, out +
                    "(" + str.substring(index, i + 1) + ")");
    }

    public static void main(String[] args) {
        // input string
        String str = "ab";
        findCombinations(str, 0, "");
    }
}

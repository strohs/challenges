package algorithms.easy

/*
Given an array of integers, find and print the maximum number of integers you can select from the array such that
the absolute difference between any two of the chosen integers (in the set) is less than or equal to 1.
For example, if your array is a = [1,1,2,2,4,4,5,5,5], you can create two sub-arrays meeting the
criterion: [1,1,2,2] and [4,4,5,5,5]. The maximum length subarray has 5 elements.
 */


/**
 * @param a: the array of integers
 * @return the max number of integers selected from "a" such that the absolute difference between any two integers
 * is <= 1
 * constraints:
 *      answer will be >= 2,
 *      2 <= array.length <= 100
 *      0 < a[i] < 100
 */
fun pickingNumbers(a: Array<Int>): Int {
    fun absDiff(n1: Int, n2: Int) : Boolean = Math.abs(n1 - n2) <= 1

    val sl = a.sorted()
    val resList = mutableListOf<List<Int>>()
    var curMax = 0
    sl.forEachIndexed { index, n ->
        val matching = sl.drop(index).takeWhile { absDiff(n, it) }
        if (matching.size > 1 && matching.size > curMax) {
            curMax = matching.size
            resList.add( matching )
            println("matching list: $matching with size: ${matching.size}")
        }
    }
    return curMax
}

fun main() {
    val arr1 = arrayOf(1,1,2,2,4,4,5,5,5)
    val res = pickingNumbers(arr1)
    println("res: $res")

}
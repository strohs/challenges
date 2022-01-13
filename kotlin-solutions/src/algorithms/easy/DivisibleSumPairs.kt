package algorithms.easy

/*
You are given an array of n integers, ar = [ar[0],ar[1]],... ar[n-1] ] and a positive integer, k.
Find and print the number of (i,j) pairs where i < j and  ar[i]+ar[j] is divisible by k

For example, ar = [1,2,3,4,5,6] and k = 5   Our three pairs meeting the criteria are [1,4],[2,3] and [4,6]
 */

/**
 * n: the integer length of array: ar
 * ar: an array of integers
 * k: the integer to divide the pair sum by
 *
 * @return Print the number of pairs (i,j) where i < j and a[i] + a[j] is evenly divisible by
 */
fun divisibleSumPairs(n: Int, k: Int, ar: Array<Int>): Int {

    var ars = ar.sortedArray().toList()
    var sumPairCount = 0

    while (ars.isNotEmpty()) {
        val elem = ars.first()
        sumPairCount += ars.drop(1).filter { (elem + it) % k == 0 }.size
        ars = ars.drop(1)
    }
    return sumPairCount
}
fun buildPairs(ars: Array<Int>): List<List<Int>> {
    val pairs = mutableListOf<List<Int>>()
    for (i in 0 until ars.size - 1) {
        for (j in i + 1 until ars.size) {
            pairs.add( listOf( ars[i],ars[j]) )
        }
    }
    return pairs
}

fun main() {
    val arr = arrayOf(1,2,3,4,5,6)
    println( divisibleSumPairs( arr.size, 5, arr))

}
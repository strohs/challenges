package algorithms.easy

/*
John works at a clothing store. He has a large pile of socks that he must pair by color for sale. Given an array of
integers representing the color of each sock, determine how many pairs of socks with matching colors there are.

For example, there are n = 7 socks with colors ar=[1,2,1,2,1,3,2]. There is one pair of color 1 and one of color 2.
There are three odd socks left, one of each color. The number of pairs is 2.
 */

/**
 *
 * @param n: the number of socks in the pile
 * @param ar: the colors of each sock

 * @return an integer representing the number of matching pairs of socks that are available
 */
fun sockMerchant(n: Int, ar: Array<Int>): Int {
    return ar.groupingBy { it }
            .eachCount()
            .mapValues { (_,v) -> v / 2 }
            .values
            .reduce { acc,v -> acc + v }
}

fun main() {
    val arr = arrayOf(1,2,1,2,1,3,2)
    val arr2 = arrayOf(1,2,1,2,1,2,1,2,3,4)
    println( sockMerchant(arr.size, arr))
    println( sockMerchant(arr2.size, arr2))
}
package algorithms.easy

/**
Given five positive integers, find the minimum and maximum values that can be calculated by summing
exactly four of the five integers. Then print the respective minimum and maximum values as a single
line of two space-separated long integers.
 **/

fun miniMaxSum(arr: Array<Int>): Unit {
    arr.sort()
    val min = arr.take(4).fold(0L) { acc, i -> acc + i.toLong() }
    val max = arr.takeLast(4).fold(0L) { acc, i -> acc + i.toLong() }
    println("$min $max")
}

fun main() {
    val tarr = arrayOf(5,2,3,1,4)
    miniMaxSum(tarr)
}
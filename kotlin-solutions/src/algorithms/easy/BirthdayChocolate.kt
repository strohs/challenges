package algorithms.easy

/*
Lily has a chocolate bar that she wants to share with Ron for his birthday. Each of the squares has an integer on it.
She decides to share a contiguous segment of the bar selected such that the length of the segment matches Ron's birth
month and the sum of the integers on the squares is equal to his birth day. You must determine how many ways she can
divide the chocolate.

Consider the chocolate bar as an array of squares, s = [2,2,1,3,2]
She wants to find segments summing to Ron's birth day, d = 4 with a length equalling his birth month, m = 2
i.e. Lily wants to give Ron 2 contiguous squares that sum to 4
In this case, there are two segments meeting her criteria: [2,2] and [1,3]
 */

/**
 * birthday has the following parameter(s):
 *  s: an array of integers, the numbers on each of the squares of chocolate
 *  d: an integer, Ron's birth day
 *  m: an integer, Ron's birth month
 *
 * @return an integer denoting the total number of ways that Lily can portion her chocolate bar to share with Ron.
 */
fun birthday(s: Array<Int>, d: Int, m: Int): Int {
    return s.toList()
            .windowed(m)
            .filter { it.sum() == d }
            .size
}

fun main() {
    val arr = arrayOf(2,2,1,3,2)
    println( birthday(arr, 4, 2) )
}
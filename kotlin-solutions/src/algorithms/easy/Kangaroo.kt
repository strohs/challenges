package algorithms.easy

/*
You are choreographing a circus show with various animals. For one act, you are given two kangaroos on a number
line ready to jump in the positive direction (i.e, toward positive infinity).

    * The first kangaroo starts at location x1 and moves at a rate of v1 meters per jump.
    * The second kangaroo starts at location x2 and moves at a rate of v2 meters per jump.

You have to figure out a way to get both kangaroos at the same location at the same time as part of the show.
If it is possible, return YES, otherwise return NO.

Constraints:
    0 <= x1 < x2 <= 10000
 */

fun kangaroo(x1: Int, v1: Int, x2: Int, v2: Int): String {
    var meet = false
    var k1 = x1     // kangaroo1 position
    var k2 = x2     // kangaroo2 position
    while (k1 <= k2 && !meet) {
        if (k1 == k2) {
            meet = true
        } else {
            k1 += v1
            k2 += v2
        }
    }
    return if (meet) "YES" else "NO"
}


fun main() {
    println( kangaroo(0,3,4,2) )

}
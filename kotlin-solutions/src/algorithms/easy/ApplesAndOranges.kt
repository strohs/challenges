package algorithms.easy


/**
 * should print the number of apples and oranges that land on Sam's house, each on a separate line.
 *   countApplesAndOranges has the following parameter(s):
 *      s: integer, starting point of Sam's house location.
 *      t: integer, ending location of Sam's house location.
 *      a: integer, location of the Apple tree.
 *      b: integer, location of the Orange tree.
 *      apples: integer array, distances at which each apple falls from the tree.
 *      oranges: integer array, distances at which each orange falls from the tree.
 */
fun countApplesAndOranges(s: Int, t: Int, a: Int, b: Int, apples: Array<Int>, oranges: Array<Int>): Unit {
    val houseRange = s..t

    val applesOnHouse = apples.filter { pos -> (a + pos) in houseRange }
    val orangesOnHouse = oranges.filter { pos -> (b + pos) in houseRange }

    println(applesOnHouse.size)
    println(orangesOnHouse.size)
}

fun main() {
  countApplesAndOranges(7, 11, 5, 15, arrayOf(-2,2,1), arrayOf(5,-6))
}
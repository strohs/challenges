package algorithms.easy

/*
You will be given two arrays of integers and asked to determine all integers that satisfy the following two conditions:

    * The elements of the first array are all factors of the integer being considered
    * The integer being considered is a factor of all elements of the second array

These numbers are referred to as being between the two arrays. You must determine how many such numbers exist.
For example, given the arrays a = [2,6] and b = [24,36] there are two numbers between them: 6 and 12

Output:
Print the number of integers that are considered to be between  a  and  b

Constraints: the two input arrays will contain integers between 1 and 100, and their size will be <= 10
 */

fun getTotalX(a: Array<Int>, b: Array<Int>): Int {
    fun allElementsAreFactors( n: Int, ns: Array<Int> ): Boolean = ns.all { n % it == 0 }
    fun intIsFactorOfAllElements( n: Int, ns: Array<Int> ): Boolean = ns.all { it % n == 0 }

    // find the minimum and max values of the integers to consider for evaluation
    val min = a.maxOrNull()!!
    val max = b.maxOrNull()!!
    val ints = min..max

    val betweenInts = ints.filter { allElementsAreFactors(it, a) && intIsFactorOfAllElements(it, b) }
    return betweenInts.size
}

fun main() {
    val a = arrayOf(2,4)
    val b = arrayOf(16,32,96)
    println(getTotalX(a,b))
}
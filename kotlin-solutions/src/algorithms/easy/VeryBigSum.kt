package algorithms.easy

import java.math.BigInteger

// Complete the aVeryBigSum function below.
fun aVeryBigSum(ar: Array<Long>): Long {
    val sum: BigInteger = ar.fold(BigInteger.ZERO) { acc, l ->
        acc + l.toBigInteger()
    }
    return sum.toLong()
}

fun main() {
    val sum = aVeryBigSum(arrayOf(1000000001, 1000000002, 1000000003, 1000000004, 1000000005))
    println("the sum: $sum")
}
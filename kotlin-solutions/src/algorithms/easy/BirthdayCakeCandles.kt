package algorithms.easy

/**
You are in charge of the cake for your niece's birthday and have decided the cake will have one candle for each
year of her total age. When she blows out the candles, she’ll only be able to blow out the tallest ones. Your
task is to find out how many candles she can successfully blow out.

For example, if your niece is turning 4 years old, and the cake will have candles of height 4, 4, 3, 1, she will
be able to blow out candles successfully, since the tallest candles are of height and there are such candles.
 **/

fun birthdayCakeCandles(ar: Array<Int>): Int {
    ar.sort()
    val tallest = ar.takeLastWhile { it == ar.last() }
    return tallest.size
}

fun main() {
    val ar1 = arrayOf(4,4,3,1,3,5)
    println(birthdayCakeCandles(ar1))
}
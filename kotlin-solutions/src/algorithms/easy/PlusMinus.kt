package algorithms.easy

/*
Given an array of integers, calculate the fractions of its elements that are positive, negative, and are zeros.
Print the decimal value of each fraction on a new line.

For example given the array [1, 1, 0, -1, -1] there are 5 elements, 2 positive, 2 negative, 1 zero
Their ratios would be 2/5 = 0.400000, 2/5 = 0.400000, 1/5 = 0.20000

Note: This challenge introduces precision problems. The test cases are scaled to six decimal places, though
answers with absolute error of up to are acceptable.
 */



fun plusMinus(arr: Array<Int>): Unit {
    fun format6(n: Double) : String = "%.6f".format(n)

    val arrSize = arr.size.toDouble()

    var posCount = 0
    var negCount = 0
    var zeroCount = 0
    arr.forEach {
        when {
            it > 0 -> posCount++
            it < 0 -> negCount++
            else   -> zeroCount++
        }
    }
    println(format6(posCount / arrSize))
    println(format6(negCount / arrSize))
    println(format6(zeroCount / arrSize))
}

fun main() {
    val arr1 = arrayOf(-4,3,-9,0,4,1)
    plusMinus(arr1)
}
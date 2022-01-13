package algorithms.easy

// Complete the staircase function below.
fun staircase(n: Int): Unit {
    for (i in n - 1 downTo 0) {
        val str = "#".repeat(n - i)
        println(str.padStart(n,' '))
    }

}

fun main() {
    staircase(100)
    val ss = "fod"
    ss.format()
}
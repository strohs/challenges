package algorithms.easy

/*
HackerLand University has the following grading policy:

 * Every student receives a grade in the inclusive range from 0 to 100
 * any grade less than 40 is a failing grade

Sam is a professor at the university and likes to round each student's grade according to these rules:
    * If the difference between the grade and the next multiple of 5 is less than 3, round grade up to the
      next multiple of 5
    * If the value of grade is less than 38, no rounding occurs as the result will still be a failing grade

For example, grade=84 will be rounded to 85 but grade=29 will not be rounded because the rounding would result
in a number that is less than 40.

Given the initial value of grade for each of Sam's N students, write code to automate the rounding process.

Constraints
-----------
* 1 <= n <= 60
* 0 <= grades[i] <= 100

 */
fun roundTo5(n :Int) : Int {
    return when(val rem = n % 5) {
        0 -> n
        else -> (5 - rem) + n
    }
}

fun gradingStudents(grades: Array<Int>): Array<Int> {
    return grades.map {
        when {
            it < 38 -> it
            else -> {
                val rg = roundTo5(it)
                if (rg - it < 3)
                    rg
                else
                    it
            }
        }
    }.toTypedArray()
}

fun main() {
    val arr1 = arrayOf(73,67,38,33)
    val rarr = gradingStudents(arr1)
    rarr.forEach { println(it) }
}
package algorithms.easy

/*
Maria plays college basketball and wants to go pro. Each season she maintains a record of her play. She tabulates
the number of times she breaks her season record for most points and least points in a game. Points scored in the
first game establish her record for the season, and she begins counting from there.

For example, assume her scores for the season are represented in the array scores = [12,24,10,24]

Given Maria's scores for a season, find and print the number of times she breaks her records for most and least
points scored during the season.
 */

/**
 * Complete the breakingRecords function in the editor below. It must return an integer array containing the numbers
 * of times she broke her records. Index 0 is for breaking most points records, and index 1 is for breaking least
 * points records.
 */
fun breakingRecords(scores: Array<Int>): Array<Int> {

    var curMax = scores[0]
    var curMin = scores[0]
    val res = scores.fold(arrayOf(0,0)) { acc, score ->
        if (score > curMax) {
            acc[0]++
            curMax = score
        }
        if (score < curMin) {
            acc[1]++
            curMin = score
        }
        acc
    }
    return res
}

fun main() {
    val arr1 = arrayOf(10, 5, 20, 20, 4, 5, 2, 25, 1)
    val res = breakingRecords(arr1)
    println( "${res[0]}  ${res[1]}" )
}
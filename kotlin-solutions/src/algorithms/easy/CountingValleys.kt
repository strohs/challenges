package algorithms.easy

/*
Gary is an avid hiker. He tracks his hikes meticulously, paying close attention to small details like topography.
During his last hike he took exactly n steps. For every step he took, he noted if it was an uphill, U, or a
downhill, D step. Gary's hikes start and end at sea level and each step up or down represents a 1 unit change in
altitude. We define the following terms:
    * A mountain is a sequence of consecutive steps above sea level, starting with a step up from sea level and
    ending with a step down to sea level.
    * A valley is a sequence of consecutive steps below sea level, starting with a step down from sea level and
    ending with a step up to sea level.

Given Gary's sequence of up and down steps during his last hike, find and print the number of valleys he walked through.
For example, if Gary's path is s=[DDUUUUDD], he first enters a valley 2 units deep. Then he climbs out an up onto a
mountain 2 units high. Finally, he returns to sea level and ends his hike.
 */

/**
 * @param n: the number of steps Gary takes   2 <= n <= 10^6
 * @param s: a string describing his path     contains a sequence of "U"s and/or "D"s

 * @return an integer that denotes the number of valleys Gary traversed.
 */
fun countingValleys(n: Int, s: String): Int {
    var valleyCount = 0
    var curHeight = 0
    s.forEach { step ->
        when (step) {
            'U' -> curHeight += 1
            'D' -> curHeight -= 1
        }
        if (curHeight == 0 && step == 'U') valleyCount += 1
    }
    return valleyCount
}

fun main() {
    val s = "DDUUUUDD"
    val s1 = "UDDDUDUU"
    val s3 = "DUDU"
    val s4 = "UUUUDDUUDDDD"
    println( countingValleys( s.length, s) )
    println( countingValleys( s1.length, s1) )
    println( countingValleys( s3.length, s3) )
    println( countingValleys( s4.length, s4) )

}
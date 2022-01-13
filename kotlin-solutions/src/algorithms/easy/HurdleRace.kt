package algorithms.easy

/// Dan is playing a video game in which his character competes in a hurdle race. Hurdles are of
/// varying heights, and Dan has a maximum height he can jump. There is a magic potion he can take
/// that will increase his maximum height by 1 unit for each dose. How many doses of the potion
/// must he take to be able to jump all of the hurdles.
///
/// Given an array of hurdle heights, height, and an initial maximum height Dan can jump, k,
/// determine the minimum number of doses Dan must take to be able to clear all the hurdles in the
/// race.
///
/// # Example
/// if height=[1,2,3,3,2] and Dan can jump 1 unit high naturally, he must take 3 - 1 = 2 doses
/// of potion to be able to jump all of the hurdles.


/**
 * @return the minimum units of potion Dan needs to drink to jump all of the hurdles.
 *
 * @param k an integer denoting the height Dan can jump naturally
 * @param height an array of integers denoting the heights of each hurdle
 */
fun hurdleRace(k: Int, height: Array<Int>): Int {
    val maxHurdle = height.maxOrNull()!!
    return if (maxHurdle > k) maxHurdle - k else 0
}

fun hurdleRace2(k: Int, height: Array<Int>): Int {
    var maxJump = k
    var doseCount = 0

    for (h in height) {
        if (h > maxJump) {
            println("cur $h $maxJump $doseCount")
            doseCount += h - maxJump
            maxJump += doseCount
            println("new $h $maxJump $doseCount")
        }
    }
    return doseCount
}

fun main() {
    println( hurdleRace(4, arrayOf(1,6,5,3,2)))
    println( hurdleRace(7, arrayOf(2,6,4,5,2)))
    val tstr = "86 4 83 20 6 81 58 59 53 2 54 62 25 35 79 64 27 49 32 95 100 20 58 39 92 30 67 89 58 81 100 66 73 29 75 81 70 55 18 28 7 35 98 52 30 11 69 48 84 54 13 14 15 86 34 82 92 26 8 53 62 57 50 31 61 85 88 5 80 64 90 52 47 43 40 93 69 70 16 43 7 25 99 12 63 99 71 76 55 17 90 43 27 20 42 84 39 96 75 1"
    val tarr = tstr.split(" ").map { Integer.parseInt(it) }.toTypedArray()
    println( hurdleRace(53, tarr))
}
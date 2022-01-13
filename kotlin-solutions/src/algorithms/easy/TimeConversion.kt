package algorithms.easy

import java.util.regex.Matcher
import java.util.regex.Pattern

/*
 * sample input:  07:05:45PM
 * sample output: 19:05:45
 */
fun timeConversion(s: String): String {
    val pat = Pattern.compile("""(\d\d):(\d\d):(\d\d)(AM|PM)""")
    val mr: Matcher = pat.matcher(s)
    if (mr.matches()) {
        val ampm = mr.group(4)
        var hour = mr.group(1).toInt()
        when(ampm) {
            "AM" -> {
                if (hour == 12) hour = 0
            }
            "PM" -> {
                hour += 12
                if ( hour >= 24 ) hour = 12
            }
        }
        return "${String.format("%02d",hour)}:${mr.group(2)}:${mr.group(3)}"
    }
    return "NOMATCH"
}

fun main() {
    println(timeConversion("07:05:45PM"))
    println(timeConversion("01:22:33PM"))
    println(timeConversion("01:22:33AM"))
    println(timeConversion("12:00:00AM"))
    println(timeConversion("12:00:00PM"))
}
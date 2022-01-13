package algorithms.easy

/*
Marie invented a Time Machine and wants to test it by time-traveling to visit Russia on the Day of the 256th
Programmer (the day of the year) during a year in the inclusive range from 1700 to 2700

From 1700 to 1917, Russia's official calendar was the Julian calendar; since 1919 they used the Gregorian calendar
system. The transition from the Julian to Gregorian calendar system occurred in 1918, when the next day after
January 31st was February 14th. This means that in 1918, February 14th was the 32nd day of the year in Russia.

In both calendar systems, February is the only month with a variable amount of days; it has 29 days during a leap year,
and 28 days during all other years. In the Julian calendar, leap years are divisible by 4; in the Gregorian calendar,
leap years are either of the following:
    * Divisible by 400
    * Divisible by 4 and not divisible by 100
Given a year y; find the date of the 256th day of that year according to the official Russian calendar during that year.
Then print it in the format dd.mm.yyyy, where dd is the two-digit day, mm is the two-digit month, and yyyy is y

For example, the given year = 1984, 1984 is divisible by 4 so it is a leap year. The 256th day of a leap year after 1918
 is September 12, so the answer is 12.09.1984
 */


fun dayOfProgrammer(year:Int) : String {
    fun isJulianLeapYear(n: Int) : Boolean = n % 4 == 0
    fun isGregorianLeapYear(n: Int): Boolean = n % 400 == 0 || n % 4 == 0 && n % 100 != 0

    var days = 31 + 31 + 30 + 31 + 30 + 31 + 31
    days += when {
        year <= 1917 && isJulianLeapYear(year) -> 29
        year == 1918 -> 28 - 13
        year > 1918 && isGregorianLeapYear(year) -> 29
        else -> 28
    }
    return String.format("%2d.09.%4d", 256 - days, year)
}


fun main() {
    println(dayOfProgrammer(1917))
    println(dayOfProgrammer(2016))
    println(dayOfProgrammer(1918))
}
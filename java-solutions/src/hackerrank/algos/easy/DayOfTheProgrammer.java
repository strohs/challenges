package hackerrank.algos.easy;

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

import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.util.function.Predicate;

public class DayOfTheProgrammer {

    static boolean isJulianLeapYear(int n) { return n % 4 == 0; }
    static boolean isGregorianLeapYear(int n) { return ( (n % 400 == 0) || ( n % 4 == 0 && n % 100 != 0)); }

    /**
     * should return a string representing the date of the 256th day of the year given.
     *
     * @param year: an integer  1700 <= year <= 2700
     * @return a string representing the date of the 256th day of the year given.
     */
    static String dayOfProgrammer(int year) {
        int days = 31 + 31 + 30 + 31 + 30 + 31 + 31;
        if (year <= 1917 && isJulianLeapYear(year)) {
            days += 29;
        } else if (year == 1918) {
            days += 28 - 13; // 13 days disappeared during transition to Gregorian calendar in 1918
        } else if (year > 1918 && isGregorianLeapYear(year)) {
            days += 29;
        } else {
            days += 28;
        }
        return String.format("%2d.09.%4d",256 - days,year);
    }

    public static void main(String[] args) {
        System.out.println(dayOfProgrammer(2017));
        System.out.println(dayOfProgrammer(2016));
        System.out.println(dayOfProgrammer(1800));
        System.out.println(dayOfProgrammer(1918));
    }
}

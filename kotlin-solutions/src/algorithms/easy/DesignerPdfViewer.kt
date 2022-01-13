package algorithms.easy

/*
In this challenge, you will be given a list of letter heights in the alphabet and a string. Using the letter heights
given, determine the area of the rectangle highlight in mm^2 assuming all letters are 1mm wide.

For example, the highlighted  word=torn   Assume the heights of the letters are t=2, o=1, r=1 and n=1. The tallest
letter is 2 high and there are 4 letters. The hightlighted area will be 2*4 = 8mm^2 so the answer is 8
 */

/**
 *
 * @param h: an array of integers representing the heights of each letter (1 <= h[?]] <= 7 where ? is an english
 * lowercase letter)
 * @param word: a string (contains no more than 10 letters)
 *
 * @return an integer representing the size of the highlighted area.
 */
fun designerPdfViewer(h: Array<Int>, word: String): Int {
    fun charIndex(ch: Char): Int = Character.getNumericValue(ch) - Character.getNumericValue('a')

    val maxHeight = word.map { h[charIndex(it)] }.maxOrNull()!!
    return maxHeight * word.length
}



fun main() {
    val h = arrayOf(1, 3, 1, 3, 1, 4, 1, 3, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7)
    val word = "zaba"
    println("highlight size=${designerPdfViewer(h, word)}")
}
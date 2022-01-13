package algorithms.easy

/*
Brie’s Drawing teacher asks her class to open their books to a page number. Brie can either start
turning pages from the front of the book or from the back of the book. She always turns pages one
at a time. When she opens the book, page 1 is always on the right side.

When she flips page 1, she sees pages 2 and 3. Each page except the last page will always be
printed on both sides. The last page may only be printed on the front, given the length of the book.
If the book is pages n long, and she wants to turn to page p, what is the minimum number of pages
she will turn? She can start at the beginning or the end of the book.

Given n and p, find and print the minimum number of pages Brie must turn in order to arrive
at page p.
*/

/**
 * @param n the total number of pages in the book
 * @param p the page you want to turn to
 * @return the minimum number of pages you need to turn to reach the target page: p, you can
 * start at the front of the book, or the back of the book
 */
fun pageCount(n: Int, p: Int): Int {
    fun isEven(n: Int) = n % 2 == 0
    fun turnsFromStart(targetPage: Int): Int = targetPage / 2
    fun turnsFromEnd(totPages :Int, targetPage:Int): Int = (totPages/2) - (targetPage/2)
    return minOf( turnsFromStart(p), turnsFromEnd(n,p) )
}

fun main() {
    println(pageCount(6,2))
    println(pageCount(5,4))
    println(pageCount(5,5))
    println(pageCount(6,6))
    println(pageCount(20,4))
    println(pageCount(20,17))
}
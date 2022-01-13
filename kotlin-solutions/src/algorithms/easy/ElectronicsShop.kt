package algorithms.easy

import java.io.File

/*
Monica wants to buy a keyboard and a USB drive from her favorite electronics store. The store has several models of
each. Monica wants to spend as much as possible for the 2 items, given her budget.

Given the price lists for the store's keyboards and USB drives, and Monica's budget, find and print the amount of
money Monica will spend. If she doesn't have enough money to both a keyboard and a USB drive, print -1 instead.
She will buy only the two required items.

For example, suppose she has to spend 60. Three types of keyboards cost kb=[40,50,60]. Two USB drives cost udb=[5,8,12].
She could purchase a 40 keyboard + 12 usb drive = 52, or a 50 keyboard + 8 usb drive. She chooses the latter.
She can't buy more than 2 items so she can't spend exactly.
 */

fun getMoneySpent(keyboards: Array<Int>, drives: Array<Int>, b: Int): Int {
    fun closestPairToSum(l1: List<Int>, l2: List<Int>, sum: Int): List<Int> {
        var closestPair= emptyList<Int>()
        var curClosest = Int.MAX_VALUE
        for (i1 in l1) {
            val i2 = l2.find { item -> i1 + item <= sum && sum - (i1 + item) < curClosest }
            if (i2 != null) {
                curClosest = sum - (i1 + i2)
                closestPair = listOf(i1,i2)
            }
        }
        return closestPair
    }

    val ks = keyboards.filter { it < b}.sortedDescending()
    val ds = drives.filter { it < b }.sorted()

    if (ks.isEmpty() || ds.isEmpty()) return -1
    val items: List<Int>
    items = if (ks.first() >= ds.first()) {
        closestPairToSum(ks,ds, b)
    } else {
        closestPairToSum(ds,ks,b)
    }
    return if (items.isNotEmpty()) items.sum() else -1
}

fun main() {
    val spent = getMoneySpent(arrayOf(3,1), arrayOf(5,2,8), 10)
    println("$spent")

    val spent2 = getMoneySpent(arrayOf(5,6,4), arrayOf(7,8,9), 10)
    println("$spent2")

    val spent3 = getMoneySpent(arrayOf(4), arrayOf(5), 5)
    println("$spent3")

    val lines = File("elect-shop.txt").readLines()
    // 539855 818 628
    val kbs = lines[0].split(" ").map { it.trim().toInt() }.toTypedArray()
    val dbs = lines[1].split(" ").map { it.trim().toInt() }.toTypedArray()
    val spent4 = getMoneySpent(kbs, dbs, 539855)
    println(spent4) // 539007  expected: 539854
}
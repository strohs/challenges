package algorithms.easy

/*
We define a magic square to be an N x N matrix of distinct positive integers from 1 to n^2 where the sum of any row,
column, or diagonal of length is always equal to the same number: the magic constant.

You will be given a 3x3 matrix of integers in the inclusive range [1..9] . We can convert any digit a to any other
digit b in the range [1,9] at cost of |a - b|. Given S, convert it into a magic square at MINIMAL COST.
Print this cost on a new line.
NOTE: the resulting magic square must contain distinct integers in the inclusive range [1..9]
p.s.: the magic constant M = n * (n^2 + 1)/ 2
p.p.s: Excluding rotations and reflections, there is exactly one 3×3 magic square

For Example: given the following matrix s:
5 3 4
1 5 8
6 4 2

can be converted to the magic square:
8 3 4
1 5 9
6 7 2

this took three replacements at a cost of |5 - 8| + |8 - 9| + |4 - 7| = 7
 */


val transforms = build3x3Transforms()

/**
 * @return the cost to convert the square matrix s, into a magic square
 */
fun formingMagicSquare(s: Array<Array<Int>>): Int {
    fun computeCost(m1: Array<Array<Int>>, m2: Array<Array<Int>>): Int {
        var cost = 0
        for (row in 0 until m1.size) {
            for (col in 0 until m1.size) {
                cost += Math.abs( m1[row][col] - m2[row][col] )
            }
        }
        return cost
    }
    //val transforms = build3x3Transforms()
    val costs = transforms.map { t -> computeCost(s,t) }
    println("costs:${costs.joinToString()}")

    return costs.minOrNull()!!
}

// build list of all reflections and rotations of the 3x3 magic square
fun build3x3Transforms(): List<Array<Array<Int>>> {
    val ms = arrayOf( arrayOf(8,1,6), arrayOf(3,5,7), arrayOf(4,9,2))
    val m1 = arrayOf( arrayOf(6,1,8), arrayOf(7,5,3), arrayOf(2,9,4))
    val m2 = arrayOf( arrayOf(2,7,6), arrayOf(9,5,1), arrayOf(4,3,8))
    val m3 = arrayOf( arrayOf(4,3,8), arrayOf(9,5,1), arrayOf(2,7,6))
    val m4 = arrayOf( arrayOf(2,9,4), arrayOf(7,5,3), arrayOf(6,1,8))
    val m5 = arrayOf( arrayOf(4,9,2), arrayOf(3,5,7), arrayOf(8,1,6))
    val m6 = arrayOf( arrayOf(6,7,2), arrayOf(1,5,9), arrayOf(8,3,4))
    val m7 = arrayOf( arrayOf(8,3,4), arrayOf(1,5,9), arrayOf(6,7,2))
    return listOf(ms,m1,m2,m3,m4,m5,m6,m7)
}

// rotate a square matrix 90 degrees clockwise, returns a new matrix
fun rotate90(s: Array<Array<Int>>): Array<Array<Int>> {
    val order = s[0].size
    val ret: Array<Array<Int>> = Array(order) { Array(order) {0} }

    for (i in 0 until order) {
        for (j in 0 until order) {
            ret[i][j] = s[order - j - 1][i]
        }
    }
    return ret
}

// reflect by the middle column(s) of the matrix s
fun reflectColumn(s: Array<Array<Int>>): Array<Array<Int>> {
    val order = s[0].size
    val ret: Array<Array<Int>> = Array(order) { Array(order) {0} }
    for (i in 0 until order) {
        for (j in 0 until order) {
            ret[i][j] = s[i][order - j - 1]
        }
    }
    return ret
}

fun reflectRow(s: Array<Array<Int>>): Array<Array<Int>> {
    val order = s[0].size
    val ret: Array<Array<Int>> = Array(order) { Array(order) {0} }

    for (i in 0 until order) {
        for (j in 0 until order) {
            ret[i][j] = s[order - i -1][j]
        }
    }
    return ret
}

// reflect across the diagonal from Top Left to Bottom Right
fun reflectDiagTB(s: Array<Array<Int>>): Array<Array<Int>> {
    val order = s[0].size
    val ret: Array<Array<Int>> = Array(order) { Array(order) {0} }

    for (i in 0 until order) {
        for (j in 0 until order) {
            ret[i][j] = s[j][i]
        }
    }
    return ret
}

fun reflectDiagBT(s: Array<Array<Int>>): Array<Array<Int>> {
    val order = s[0].size
    val ret: Array<Array<Int>> = Array(order) { Array(order) {0} }

    for (i in 0 until order) {
        for (j in 0 until order) {
            ret[i][j] = s[order - j - 1][order - i - 1]
        }
    }
    return ret
}

fun printMatrix(s: Array<Array<Int>>) = s.forEach { row -> println( row.joinToString() )}

fun main() {
    val s1 = arrayOf( arrayOf(4,9,2), arrayOf(3,5,7), arrayOf(8,1,5))
    val s2 = arrayOf( arrayOf(4,8,2), arrayOf(4,5,7), arrayOf(6,1,6))
    println( formingMagicSquare(s1) )
    println( formingMagicSquare(s2) )
}
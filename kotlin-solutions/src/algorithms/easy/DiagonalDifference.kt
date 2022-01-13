package algorithms.easy

val matrix1 = arrayOf(
        arrayOf(-1, 1, -7, -8),
        arrayOf(-10, -8, -5, -2),
        arrayOf(0, 9, 7, -1),
        arrayOf(4, 4, -2, 1)
)

fun diagonalDifference(arr: Array<Array<Int>>): Int {
    // arr will always be a square matrix
    val length = arr[0].size
    var lrDiag = Array(length) {0}
    var rlDiag = Array(length) {0}
    var cIdx = 0

    for (r in 0 until length)
        lrDiag[r] = arr[r][cIdx++]
    cIdx = length - 1
    for (r in 0 until length)
        rlDiag[r] = arr[r][cIdx--]
    return Math.abs(lrDiag.sum() - rlDiag.sum())
}

fun main() {
    val dd = diagonalDifference(matrix1)
    println("diag diff: $dd")
}
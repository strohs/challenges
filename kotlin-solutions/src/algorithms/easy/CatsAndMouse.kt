package algorithms.easy

/*
Two cats and a mouse are at various positions on a line. You will be given their starting positions. Your task is
to determine which cat will reach the mouse first, assuming the mouse doesn't move and the cats travel at equal speed.
If the cats arrive at the same time, the mouse will be allowed to move and it will escape while they fight.

You are given q queries in the form of x, y, and z representing the respective positions for cats A and B, and for
mouse C. Complete the function "catAndMouse" to return the appropriate answer to each query, which will be printed
on a new line.
    * If cat A catches the mouse first, print "Cat A".
    * If cat B catches the mouse first, print "Cat B".
    * If both cats reach the mouse at the same time, print "Mouse C" as the two cats fight and mouse escapes

For example, cat A is at position x=2 and cat B is at y=5. If mouse C is at position z=4, it is 2 units from cat A
and 1 unit from cat B. Cat B will catch the mouse.
 */

/**
 * should return one of the three strings as described.
 *
 * catAndMouse has the following parameter(s):
 * @param x: an integer, Cat's position
 * @param y: an integer, Cat's position
 * @param z: an integer, Mouse's position
 *
 * 1 <= x,y,x <= 100
 */
fun catAndMouse(x: Int, y: Int, z: Int): String {
    val xDist = Math.abs(x - z)
    val yDist = Math.abs(y - z)

    return when {
        xDist < yDist -> "Cat A"
        yDist < xDist -> "Cat B"
        else -> "Mouse C"
    }
}

fun main() {
    println( catAndMouse(1,2,3) )
    println( catAndMouse(1,3,2) )
}
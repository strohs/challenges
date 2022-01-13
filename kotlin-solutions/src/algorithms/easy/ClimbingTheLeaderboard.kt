package algorithms.easy

import java.io.File
import java.util.*

/*
Alice is playing an arcade game and wants to climb to the top of the leaderboard and wants to track her ranking.
The game uses Dense Ranking, so its leaderboard works like this:
    * The player with the highest score is ranked number 1 on the leaderboard.
    * Players who have equal scores receive the same ranking number, and the next player(s) receive the immediately
    following ranking number.

For example, the four players on the leaderboard have high scores of 100, 90, 90, and 80. Those players will
have ranks 1, 2, 2, and 3, respectively. If Alice's scores are 70,80 and 105, her rankings after each game are 4th, 3rd
and 1st.

 */

/**
 * computes Alice's rank after every game
 * @param scores an Int array of leaderboard scores. This will be in descending order
 * @param alice an Int array of Alice's scores. This will be in ASCENDING order
 * @return an integer array where each element res[j] represents Alice's rank after the jth game
 */
fun climbingLeaderboardFunctional(scores: Array<Int>, alice: Array<Int>): Array<Int> {
    val scoreList = scores.toSortedSet().toList()

    return alice
            .map { aliceScore ->
                val sidx = scoreList.binarySearch(aliceScore)
                //println("sidx for score:$aliceScore :$sidx")
                when {
                    sidx >= 0 -> {
                        if (sidx == 0) scoreList.size
                        else scoreList.size - sidx
                    }
                    else -> {
                        (scoreList.size - (sidx * -1)) + 2
                    }
                }

            }
            .toTypedArray()
}

// this is the iterative solution that could time out on big inputs
fun climbingLeaderboard(scores: Array<Int>, alice: Array<Int>): Array<Int> {
    val sls = buildRankList(scores)
    var s = sls.size - 1
    var a = 0
    val res = mutableListOf<Int>()
    while (a < alice.size && s >= 0) {
        when {
            alice[a] < sls[sls.size - 1] -> {
                res.add(sls.size + 1)
                a++
            }
            alice[a] == sls[sls.size - 1] -> {
                res.add(sls.size)
                a++
            }
            alice[a] >= sls[0] -> {
                res.add(1)
                a++
            }
            alice[a] == sls[s] -> {
                res.add(s + 1)
                a++
            }
            alice[a] < sls[s] -> {
                res.add(s + 2)
                a++
            }
            else -> {
                s--
            }
        }
    }
    return res.toTypedArray()
}

fun buildRankList(scores: Array<Int>): List<Int> {
    var cidx = 0
    val groupedScores = mutableListOf<Int>()
    while (cidx < scores.size) {
        val curVal = scores[cidx]
        val gSize = scores.sliceArray(cidx until scores.size)
                .takeWhile { it == curVal }
                .size
        groupedScores.add(curVal)
        cidx += gSize
    }
    return groupedScores
}

val testScores = """997 981 957 933 930 927 926 920 916 896 887 874 863 863 858 847 815 809 803 794 789 785 783 778 764 755 751 740 737 730 691 677 652 650 587 585 583 568 546 541 540 538 531 527 506 493 457 435 430 427 422 422 414 404 400 394 387 384 374 371 369 369 368 365 363 337 336 328 325 316 314 306 282 277 230 227 212 199 179 173 171 168 136 125 124 95 92 88 85 70 68 61 60 59 44 43 28 23 13 12"""

val testAlice = """12 20 30 32 35 37 63 72 83 85 96 98 98 118 122 125 129 132 140 144 150 164 184 191 194 198 200 220 228 229 229 236 238 246 259 271 276 281 283 287 300 302 306 307 312 318 321 325 341 341 341 344 349 351 354 356 366 369 370 379 380 380 396 405 408 417 423 429 433 435 438 441 442 444 445 445 452 453 465 466 467 468 469 471 475 482 489 491 492 493 498 500 501 504 506 508 523 529 530 539 543 551 552 556 568 569 571 587 591 601 602 606 607 612 614 619 620 623 625 625 627 638 645 653 661 662 669 670 676 684 689 690 709 709 710 716 724 726 730 731 733 737 744 744 747 757 764 765 765 772 773 774 777 787 794 796 797 802 805 811 814 819 819 829 830 841 842 847 857 857 859 860 866 872 879 882 895 900 900 903 905 915 918 918 922 925 927 928 929 931 934 937 955 960 966 974 982 988 996 996"""

fun main() {
    val testPath1 = "/home/cliff/idea-projects/hackerrank/kotlin-solutions/src/main/resources/leaderBoardTestScores1.txt"
    val testAlice1 = "/home/cliff/idea-projects/hackerrank/kotlin-solutions/src/main/resources/leaderBoardTestAlice1.txt"
    val scores: Array<Int> = File(testPath1).bufferedReader().readLines().flatMap { line ->
        line.split(" ").map { Integer.parseInt(it) }
    }.toTypedArray()
    val alice = File(testAlice1).bufferedReader().readLines().flatMap { line ->
        line.split(" ").map { Integer.parseInt(it) }
    }.toTypedArray()
//    val scores = arrayOf(100,100,50,40,40,20,10)
//    val alice = arrayOf(5,25,50,120)

    val res = climbingLeaderboardFunctional(scores, alice)
    res.forEach { println(it) }
}
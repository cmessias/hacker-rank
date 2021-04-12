// https://www.hackerrank.com/challenges/jumping-on-the-clouds

fun jumpingOnClouds(c: Array<Int>): Int {
    val (njumps, sinceLastJump) = c.fold(Pair(0, 0)) { acc: Pair<Int, Int>, cloud: Int ->
        if (cloud == 1 || acc.second >= 1) Pair(acc.first + 1, 0)
        else Pair(acc.first, acc.second + 1)
    }

    return njumps
}

assert(jumpingOnClouds(arrayOf(0, 0, 1, 0, 0, 1, 0)) == 4)
assert(jumpingOnClouds(arrayOf(0, 0, 0, 0, 1, 0)) == 3)
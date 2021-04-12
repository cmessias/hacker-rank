// https://www.hackerrank.com/challenges/counting-valleys/

fun countingValleys(steps: Int, path: String): Int {
    val (nValleys, _depth) = path.fold(Pair(0, 0), { acc, step ->
        val toDepth = if (step == 'U') 1 else -1
        val newValley = if (acc.second == 0 && toDepth == -1) 1 else 0
        Pair(acc.first + newValley, acc.second + toDepth)
    })

    return nValleys
}

assert(countingValleys(8, "UDDDUDUU") == 1)

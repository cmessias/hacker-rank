// https://www.hackerrank.com/challenges/ctci-connected-cell-in-a-grid
// IO boilerplate provided by hacker rank

import kotlin.math.max

data class Point(val i: Int, val j: Int)

fun maxRegion(grid: Array<Array<Int>>): Int {
    val visited = HashSet<Point>()
    var biggestComponent = 0

    for ((i, row) in grid.withIndex()) {
        for ((j, cell) in row.withIndex()) {
            if (cell == 1 && !visited.contains(Point(i, j))) {
                biggestComponent = max(biggestComponent, regionSize(Point(i, j), grid, visited))
            }
        }
    }

    return biggestComponent
}

fun regionSize(point: Point, grid: Array<Array<Int>>, visited: HashSet<Point>): Int {
    visited.add(point)
    val toVisitNeighbors = ArrayDeque<Point>()
    toVisitNeighbors.addLast(point)

    var regionSize = 1
    var currentPoint: Point? = point

    while (currentPoint != null) {
        for (child in getPossibleChildren(currentPoint, grid)) {
            if (!visited.contains(child)) {
                regionSize += 1
                visited.add(child)
                toVisitNeighbors.addLast(child)
            }
        }
        currentPoint = toVisitNeighbors.removeFirstOrNull()
    }

    return regionSize
}

fun getPossibleChildren(point: Point, grid: Array<Array<Int>>): List<Point> {
    val (i, j) = point
    return listOf(
        Pair(-1, -1), Pair(-1, 0), Pair(-1, 1), Pair(0, -1),
        Pair(0, 1), Pair(1, -1), Pair(1, 0), Pair(1, 1)
    ).mapNotNull {
        if (i + it.first >= 0
            && i + it.first < grid.size
            && j + it.second >= 0
            && j + it.second < grid[0].size
            && grid[i + it.first][j + it.second] == 1
        ) {
            Point(i + it.first, j + it.second)
        } else {
            null
        }
    }
}

fun main() {
    val n = readLine()!!.trim().toInt()
    val m = readLine()!!.trim().toInt()
    val grid = Array(n) { Array(m) { 0 } }

    for (i in 0 until n) {
        grid[i] = readLine()!!.trimEnd().split(" ").map { it.toInt() }.toTypedArray()
    }

    val res = maxRegion(grid)
    println(res)
}

// https://www.hackerrank.com/challenges/components-in-graph/
// IO boilerplate provided by Hacker Rank
// Code golf, fewest lines wins

fun make_set(n: Int): Array<Int> {
    return (0..(2 * n)).map { it }.toTypedArray()
}

tailrec fun find(x: Int, parent: Array<Int>): Int {
    return if (x == parent[x]) { x }
    else { find(parent[x], parent) }
}

fun union(x: Int, y: Int, parent: Array<Int>) {
    val x = find(x, parent)
    val y = find(y, parent)
    if (x == y) { return; }
    parent[y] = x
}

fun componentsInGraph(p: Array<Int>): Array<Int> {
    val c = HashMap<Int, Int>()
    for (x in p) {
        c.put(find(x, p), c.getOrDefault(find(x, p), 0) + 1)
    }
    val values = c.values.filter { it > 1 }
    return arrayOf(values.minOrNull()!!, values.maxOrNull()!!)
}

val n = readLine()!!.trim().toInt()

val gb = Array<Array<Int>>(n, { Array<Int>(2, { 0 }) })

for (i in 0 until n) {
    gb[i] = readLine()!!.trimEnd().split(" ").map { it.toInt() }.toTypedArray()
}

val parent = make_set(n)

for (c in gb) {
    union(c[0], c[1], parent)
}

val result = componentsInGraph(parent)

println(result.joinToString(" "))

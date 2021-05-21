// https://www.hackerrank.com/challenges/ctci-making-anagrams/
// Code golf, fewest lines wins

import kotlin.math.absoluteValue

fun makeAnagram(a: String, b: String): Int {
    val f = HashMap<Char, Int>()
    a.groupingBy { it }.eachCount().forEach { f.put(it.key, it.value) }
    b.groupingBy { it }.eachCount().forEach { f.put(it.key, f.getOrDefault(it.key, 0) - it.value) }
    return f.values.map { it.absoluteValue }.sumOf { it }
}

val a = readLine()!!
val b = readLine()!!
val res = makeAnagram(a, b)

println(res)

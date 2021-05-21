import kotlin.math.absoluteValue

fun makeAnagram(a: String, b: String): Int {
    val frequencies = HashMap<Char, Int>()
    a.forEach { frequencies.put(it, frequencies.getOrDefault(it, 0) + 1) }
    b.forEach { frequencies.put(it, frequencies.getOrDefault(it, 0) - 1) }
    return frequencies.values.map { it.absoluteValue }.sumOf { it }
}

val a = readLine()!!
val b = readLine()!!
val res = makeAnagram(a, b)

println(res)

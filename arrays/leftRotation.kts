// https://www.hackerrank.com/challenges/ctci-array-left-rotation/

fun rotLeft(a: Array<Int>, d: Int): Array<Int> {
    return a.sliceArray(d until a.size) + a.sliceArray(0 until d)
}

fun main(args: Array<String>) {
    val first_multiple_input = readLine()!!.trimEnd().split(" ")

    val n = first_multiple_input[0].toInt()
    val d = first_multiple_input[1].toInt()
    val a = readLine()!!.trimEnd().split(" ").map{ it.toInt() }.toTypedArray()

    val result = rotLeft(a, d)

    println(result.joinToString(" "))
}

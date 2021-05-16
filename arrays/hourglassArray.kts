import java.io.*
import java.math.*
import java.security.*
import java.text.*
import java.util.*
import java.util.concurrent.*
import java.util.function.*
import java.util.regex.*
import java.util.stream.*
import kotlin.collections.*
import kotlin.comparisons.*
import kotlin.io.*
import kotlin.jvm.*
import kotlin.jvm.functions.*
import kotlin.jvm.internal.*
import kotlin.ranges.*
import kotlin.sequences.*
import kotlin.text.*

fun biggestHourglassSum(arr: Array<Array<Int>>): Int {
    var max = -63
    for (y in 0 until (arr.size - 2)) {
        for (x in 0 until (arr.size - 2)) {
            val sum = hourglassSum(y, x, arr)
            if (sum > max) {
                max = sum
            }
        }
    }
    return max
}

fun hourglassSum(y: Int, x: Int, arr: Array<Array<Int>>): Int {
    if (x + 2 < arr.size && y + 2 < arr[0].size) {
        return (arr[y].sliceArray(x..(x + 2)).sum()
                + arr[y + 1][x + 1]
                + arr[y + 2].sliceArray(x..(x + 2)).sum())
    }
    return 0;
}


val arr = Array<Array<Int>>(6, { Array<Int>(6, { 0 }) })

for (i in 0 until 6) {
    arr[i] = readLine()!!.trimEnd().split(" ").map { it.toInt() }.toTypedArray()
}

val result = biggestHourglassSum(arr)

println(result)

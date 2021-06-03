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



/*
 * Complete the 'minTime' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY files
 *  2. INTEGER numCores
 *  3. INTEGER limit
 */

fun minTime(files: Array<Int>, numCores: Int, limit: Int): Long {
    files.sort()

    var unitsOfTime: Long = 0
    var usedLimit = limit
    for (file in files) {
        if (file % numCores == 0 && usedLimit > 0) {
            unitsOfTime += file / numCores
            usedLimit -= 1
        } else {
            unitsOfTime += file
        }
    }
    return unitsOfTime
}

fun main() {
    val filesCount = readLine()!!.trim().toInt()

    val files = Array<Int>(filesCount, { 0 })
    for (i in 0 until filesCount) {
        val filesItem = readLine()!!.trim().toInt()
        files[i] = filesItem
    }

    val numCores = readLine()!!.trim().toInt()

    val limit = readLine()!!.trim().toInt()

    val result = minTime(files, numCores, limit)

    println(result)
}


main()
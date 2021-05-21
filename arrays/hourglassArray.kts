// https://www.hackerrank.com/challenges/2d-array/

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

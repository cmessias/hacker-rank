fun sockMerchant(n: Int, ar: Array<Int>): Int {
    val socksGroupedByPairs = ar.fold(HashMap<Int, Int>()) { acc, sock ->
        acc.put(sock, acc.getOrDefault(sock, 0) + 1)
        acc
    }

    return socksGroupedByPairs.values.sumBy { it / 2 }
}

assert(sockMerchant(9, arrayOf(10, 20, 20, 10, 10, 30, 50, 10, 20)) == 3)
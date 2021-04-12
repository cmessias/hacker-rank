// https://www.hackerrank.com/challenges/repeated-string

fun repeatedString(s: String, n: Long): Long {
    val repeated = n / s.length
    val remaining = n - (s.length * repeated)
    val occurencesInFullString = s.count { it == 'a' } * repeated
    val occurencesInRemainingString = s.substring(0, remaining.toInt()).count { it == 'a' }
    return occurencesInFullString + occurencesInRemainingString
}

assert(repeatedString("aba", 10) == 7L)
assert(repeatedString("a", 1000000000000) == 1000000000000L)

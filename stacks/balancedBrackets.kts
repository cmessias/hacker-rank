// https://www.hackerrank.com/challenges/balanced-brackets
// IO boilerplate provided by Hacker Rank.

import java.util.*
import kotlin.collections.*

fun isOpeningBracket(bracket: Char): Boolean {
    return bracket in listOf('[', '{', '(')
}

fun matchingBracket(left: Char, right: Char): Boolean {
    return when (left) {
        '[' -> right == ']'
        '{' -> right == '}'
        '(' -> right == ')'
        else -> false
    }
}

fun isBalanced(s: String): String {
    var stack = ArrayDeque<Char>()

    for (c in s) {
        if (isOpeningBracket(c)) {
            stack.addLast(c)
            continue
        }

        if (stack.size > 0 && matchingBracket(stack.last(), c)) {
            stack.removeLast()
            continue
        } else {
            return "NO"
        }
    }

    if (stack.size > 0) {
        return "NO"
    }

    return "YES";
}

fun main(args: Array<String>) {
    val scan = Scanner(System.`in`)

    val t = scan.nextLine().trim().toInt()

    for (tItr in 1..t) {
        val s = scan.nextLine()

        val result = isBalanced(s)

        println(result)
    }
}
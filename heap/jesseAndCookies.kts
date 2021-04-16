// https://www.hackerrank.com/challenges/jesse-and-cookies

import java.util.PriorityQueue;

fun cookies(k: Int, A: Array<Int>): Int {
    val nums = PriorityQueue<Int>();
    nums.addAll(A);

    var counter = 0;
    while (nums.size > 1 && nums.peek() < k) {
        val c1 = nums.remove();
        val c2 = nums.remove();
        nums.add(c1 + 2 * c2);
        ++counter;
    }

    return if (nums.size >= 1 && nums.peek() >= k) {
        counter
    } else {
        -1
    }
}

assert(cookies(7, arrayOf(1, 2, 3, 9, 10, 12)) == 2)